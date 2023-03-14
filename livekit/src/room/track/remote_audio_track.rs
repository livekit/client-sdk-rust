use super::TrackInner;
use crate::prelude::*;
use crate::proto;
use livekit_webrtc as rtc;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub struct RemoteAudioTrack {
    pub(crate) inner: Arc<TrackInner>,
}

impl RemoteAudioTrack {
    pub(crate) fn new(
        sid: TrackSid,
        name: String,
        rtc_track: rtc::media_stream::AudioTrack,
    ) -> Self {
        Self {
            inner: Arc::new(TrackInner::new(
                sid,
                name,
                TrackKind::Audio,
                rtc::media_stream::MediaStreamTrack::Audio(rtc_track),
            )),
        }
    }

    #[inline]
    pub fn sid(&self) -> TrackSid {
        self.inner.sid()
    }

    #[inline]
    pub fn name(&self) -> String {
        self.inner.name()
    }

    #[inline]
    pub fn kind(&self) -> TrackKind {
        self.inner.kind()
    }

    #[inline]
    pub fn source(&self) -> TrackSource {
        self.inner.source()
    }

    #[inline]
    pub fn stream_state(&self) -> StreamState {
        self.inner.stream_state()
    }

    #[inline]
    pub fn start(&self) {
        self.inner.start()
    }

    #[inline]
    pub fn stop(&self) {
        self.inner.stop()
    }

    #[inline]
    pub fn muted(&self) -> bool {
        self.inner.muted()
    }

    #[inline]
    pub fn set_muted(&self, muted: bool) {
        self.inner.set_muted(muted)
    }

    #[inline]
    pub fn rtc_track(&self) -> rtc::media_stream::AudioTrack {
        if let rtc::media_stream::MediaStreamTrack::Audio(audio) = self.inner.rtc_track() {
            audio
        } else {
            unreachable!()
        }
    }

    #[inline]
    pub fn register_observer(&self) -> mpsc::UnboundedReceiver<TrackEvent> {
        self.inner.register_observer()
    }

    #[inline]
    pub(crate) fn transceiver(&self) -> Option<rtc::rtp_transceiver::RtpTransceiver> {
        self.inner.transceiver()
    }

    #[inline]
    pub(crate) fn update_transceiver(
        &self,
        transceiver: Option<rtc::rtp_transceiver::RtpTransceiver>,
    ) {
        self.inner.update_transceiver(transceiver)
    }

    #[inline]
    pub(crate) fn update_info(&self, info: proto::TrackInfo) {
        self.inner.update_info(info)
    }
}
