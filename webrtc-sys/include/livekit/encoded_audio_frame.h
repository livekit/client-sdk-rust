#pragma once

#include <optional>

#include "livekit/frame_transformer.h"
#include "rtc_base/checks.h"

namespace livekit {
class EncodedAudioFrame;
}  // namespace livekit
#include "webrtc-sys/src/encoded_audio_frame.rs.h"

namespace livekit {

class EncodedAudioFrame {
 public:
  explicit EncodedAudioFrame(std::unique_ptr<webrtc::TransformableAudioFrameInterface> frame);

  uint32_t timestamp() const;

  uint8_t payload_type() const;
  uint16_t sequence_number() const;
  const uint8_t* payload_data() const;
  size_t payload_size() const;

  uint32_t ssrc() const;

  std::shared_ptr<uint64_t> absolute_capture_timestamp() const;
  std::shared_ptr<int64_t> estimated_capture_clock_offset() const;
  std::unique_ptr<webrtc::TransformableAudioFrameInterface> get_raw_frame();

 private:
  std::unique_ptr<webrtc::TransformableAudioFrameInterface> frame_;
  const uint8_t* data;
  size_t size;
};

}  // namespace livekit