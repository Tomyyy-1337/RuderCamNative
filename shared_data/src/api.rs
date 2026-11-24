use crate::generate_post_api;

pub const WS_URL: &str = "ws://192.168.50.1:8140/ws";
pub const API_BASE_URL: &str = "http://192.168.50.1:8140/api/";

generate_post_api!(set_focal_length, u8);
generate_post_api!(set_hdr_active, bool);
generate_post_api!(set_bitrate, u32);
generate_post_api!(set_focus_mode, crate::FocusMode);
generate_post_api!(set_metering_mode, crate::Metering);
generate_post_api!(set_exposure_compensation, f32);
generate_post_api!(toggle_fahrt, crate::SerializableDateTime);
