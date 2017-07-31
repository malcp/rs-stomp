use std::fmt::Write;

use super::header::HeaderItems;

#[derive(Debug)]
pub struct Frame {
    frame_type: &'static str,
    headers: HeaderItems,
    body: String,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            frame_type: "",
            headers: HeaderItems::new(),
            body: String::new(),
        }
    }

    pub fn set_frame_type(&self, frame_type: &'static str) -> Frame {
        Frame {
            frame_type: frame_type,
            headers: self.headers.clone(),
            body: self.body.clone(),
        }
    }

    pub fn get_frame_type(&self) -> String {
        self.frame_type.to_string()
    }

    pub fn set_headers(&self, headers: HeaderItems) -> Frame {
        Frame {
            frame_type: self.frame_type,
            headers: headers,
            body: self.body.clone(),
        }
    }

    pub fn set_body(self, body: String) -> Frame {
        Frame {
            frame_type: self.frame_type,
            headers: self.headers,
            body: body,
        }
    }

    pub fn as_string(&self) -> String {
        // TODO: do actual implementation
        let mut frame = String::new();
        writeln!(&mut frame,
                 "{frame_type}",
                 frame_type = self.get_frame_type()).unwrap();
        writeln!(&mut frame, "{}", "accept-version:1.2").unwrap();
        writeln!(&mut frame, "{}", "").unwrap();
        writeln!(&mut frame, "{body}\0", body = "").unwrap();
        frame
    }

    pub fn new_connect() -> Frame {
        // debug!("FrmConnect");
        Frame::new()
            .set_frame_type("CONNECT")
            .set_headers(HeaderItems::new()
                             .add_accept_version_default()
                             .add_host("localhost"))
    }
}
