#[cxx::bridge(namespace = "livekit")]
pub mod ffi {

    #[derive(Debug)]
    #[repr(i32)]
    pub enum MediaType {
        Audio,
        Video,
        Data,
        Unsupported,
    }

    #[derive(Debug)]
    #[repr(i32)]
    pub enum Priority {
        VeryLow,
        Low,
        Medium,
        High,
    }

    #[derive(Debug)]
    #[repr(i32)]
    pub enum RtpTransceiverDirection {
        SendRecv,
        SendOnly,
        RecvOnly,
        Inactive,
        Stopped,
    }

    unsafe extern "C++" {
        include!("livekit/webrtc.h");

        type RTCRuntime;

        fn create_rtc_runtime() -> SharedPtr<RTCRuntime>;
    }
}

unsafe impl Send for ffi::RTCRuntime {}

unsafe impl Sync for ffi::RTCRuntime {}
