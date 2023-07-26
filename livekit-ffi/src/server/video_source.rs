use crate::{proto, server, FfiError, FfiHandleId, FfiResult};
use futures_util::StreamExt;
use livekit::prelude::*;
use livekit::webrtc::prelude::*;
use livekit::webrtc::video_frame::{BoxVideoFrameBuffer, VideoFrame};
use livekit::webrtc::video_stream::native::NativeVideoStream;
use log::warn;
use tokio::sync::oneshot;

pub struct FfiVideoSource {
    handle_id: FfiHandleId,
    source_type: proto::VideoSourceType,
    source: RtcVideoSource,
}

impl FfiVideoSource {
    pub fn setup(
        server: &'static server::FfiServer,
        new_source: proto::NewVideoSourceRequest,
    ) -> FfiResult<proto::VideoSourceInfo> {
        let source_type = proto::VideoSourceType::from_i32(new_source.r#type).unwrap();
        #[allow(unreachable_patterns)]
        let source_inner = match source_type {
            #[cfg(not(target_arch = "wasm32"))]
            proto::VideoSourceType::VideoSourceNative => {
                use livekit::webrtc::video_source::native::NativeVideoSource;
                let video_source = NativeVideoSource::new(
                    new_source.resolution.map(Into::into).unwrap_or_default(),
                );
                RtcVideoSource::Native(video_source)
            }
            _ => return Err(FfiError::InvalidRequest("unsupported video source type")),
        };

        let video_source = Self {
            handle_id: server.next_id(),
            source_type,
            source: source_inner,
        };
        let source_info = proto::VideoSourceInfo::from(&video_source);

        server
            .ffi_handles
            .insert(video_source.handle_id, Box::new(video_source));

        Ok(source_info)
    }

    pub fn capture_frame(
        &self,
        server: &'static server::FfiServer,
        capture: proto::CaptureVideoFrameRequest,
    ) -> FfiResult<()> {
        match self.source {
            #[cfg(not(target_arch = "wasm32"))]
            RtcVideoSource::Native(ref source) => {
                let frame_info = capture
                    .frame
                    .ok_or(FfiError::InvalidRequest("frame is empty"))?;

                let buffer = server
                    .ffi_handles
                    .get(&capture.buffer_handle)
                    .ok_or(FfiError::InvalidRequest("handle not found"))?;

                let buffer = buffer
                    .downcast_ref::<BoxVideoFrameBuffer>()
                    .ok_or(FfiError::InvalidRequest("handle is not video frame"))?;

                let rotation = proto::VideoRotation::from_i32(frame_info.rotation).unwrap();
                let frame = VideoFrame {
                    rotation: rotation.into(),
                    timestamp_us: frame_info.timestamp_us,
                    buffer,
                };

                source.capture_frame(&frame);
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_id(&self) -> FfiHandleId {
        self.handle_id
    }

    pub fn source_type(&self) -> proto::VideoSourceType {
        self.source_type
    }

    pub fn inner_source(&self) -> &RtcVideoSource {
        &self.source
    }
}
