use super::Frame;
use super::header::HeaderItems;

pub trait FrmConnect {
    fn new_connect() -> Frame;
}

impl FrmConnect for Frame {
    fn new_connect() -> Frame {
        Frame::new()
            .set_frame_type("CONNECT")
            .set_headers(HeaderItems::new()
                             .add_accept_version_default()
                             .add_host("localhost")) as Frame
    }
}
