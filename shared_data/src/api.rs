use crate::generate_post_api;

pub const API_BASE_URL: &str = "http://127.0.0.1:3000/api/";

generate_post_api!(set_focal_length, u8);
generate_post_api!(set_hdr_active, bool);
generate_post_api!(set_bitrate, u32);
generate_post_api!(set_focus_mode, crate::FocusMode);
generate_post_api!(set_metering_mode, crate::Metering);


