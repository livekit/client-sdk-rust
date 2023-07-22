use livekit_protocol as proto;
use livekit_webrtc::prelude::*;
use log::{debug, error};
use parking_lot::Mutex;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, Mutex as AsyncMutex, Notify};
use tokio::task::JoinHandle;

use super::EngineResult;

const NEGOTIATION_FREQUENCY: Duration = Duration::from_millis(150);

pub type OnOfferCreated = Box<dyn FnMut(SessionDescription) + Send + Sync>;

struct TransportInner {
    pending_candidates: Vec<IceCandidate>,
    renegotiate: bool,
    restarting_ice: bool,
}

pub struct PeerTransport {
    signal_target: proto::SignalTarget,
    peer_connection: PeerConnection,
    on_offer_handler: Mutex<Option<OnOfferCreated>>,
    inner: Arc<AsyncMutex<TransportInner>>,
}

impl Debug for PeerTransport {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("PeerTransport")
            .field("target", &self.signal_target)
            .finish()
    }
}

impl PeerTransport {
    pub fn new(peer_connection: PeerConnection, signal_target: proto::SignalTarget) -> Self {
        Self {
            signal_target,
            peer_connection,
            on_offer_handler: Mutex::new(None),
            inner: Arc::new(AsyncMutex::new(TransportInner {
                pending_candidates: Vec::default(),
                renegotiate: false,
                restarting_ice: false,
            })),
        }
    }

    pub fn is_connected(&self) -> bool {
        matches!(
            self.peer_connection.ice_connection_state(),
            IceConnectionState::Connected | IceConnectionState::Completed
        )
    }

    pub fn peer_connection(&self) -> PeerConnection {
        self.peer_connection.clone()
    }

    pub fn signal_target(&self) -> proto::SignalTarget {
        self.signal_target.clone()
    }

    pub fn on_offer(&self, handler: Option<OnOfferCreated>) {
        *self.on_offer_handler.lock() = handler;
    }

    pub fn close(&self) {
        self.peer_connection.close();
    }

    pub async fn prepare_ice_restart(&self) {
        self.inner.lock().await.restarting_ice = true;
    }

    pub async fn add_ice_candidate(&self, ice_candidate: IceCandidate) -> EngineResult<()> {
        let mut inner = self.inner.lock().await;

        if self.peer_connection.current_remote_description().is_some() && !inner.restarting_ice {
            drop(inner);
            self.peer_connection
                .add_ice_candidate(ice_candidate)
                .await?;

            return Ok(());
        }

        inner.pending_candidates.push(ice_candidate);
        Ok(())
    }

    pub async fn set_remote_description(
        &self,
        remote_description: SessionDescription,
    ) -> EngineResult<()> {
        let mut inner = self.inner.lock().await;

        self.peer_connection
            .set_remote_description(remote_description)
            .await?;

        for ic in inner.pending_candidates.drain(..) {
            self.peer_connection.add_ice_candidate(ic).await?;
        }

        inner.restarting_ice = false;

        if inner.renegotiate {
            inner.renegotiate = false;
            self.create_and_send_offer(OfferOptions::default()).await?;
        }

        Ok(())
    }

    pub async fn create_anwser(
        &self,
        offer: SessionDescription,
        options: AnswerOptions,
    ) -> EngineResult<SessionDescription> {
        self.set_remote_description(offer).await?;
        let answer = self.peer_connection().create_answer(options).await?;
        self.peer_connection()
            .set_local_description(answer.clone())
            .await?;

        Ok(answer)
    }

    pub async fn create_and_send_offer(&self, options: OfferOptions) -> EngineResult<()> {
        let mut inner = self.inner.lock().await;

        if options.ice_restart {
            debug!("restarting ICE");
            inner.restarting_ice = false;
        }

        if self.peer_connection.signaling_state() == SignalingState::HaveLocalOffer {
            if options.ice_restart {
                if let Some(remote_description) = self.peer_connection.current_remote_description()
                {
                    self.peer_connection
                        .set_remote_description(remote_description)
                        .await?;
                } else {
                    error!("trying to restart ICE when the pc doesn't have remote description");
                }
            } else {
                inner.renegotiate = true;
                return Ok(());
            }
        }

        let offer = self.peer_connection.create_offer(options).await?;
        self.peer_connection
            .set_local_description(offer.clone())
            .await?;

        if let Some(handler) = self.on_offer_handler.lock().as_mut() {
            handler(offer);
        }

        Ok(())
    }
}
