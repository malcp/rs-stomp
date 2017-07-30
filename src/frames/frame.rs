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
            headers: self.headers,
            body: self.body,
        }
    }

    pub fn get_frame_type(&self) -> String {
        self.frame_type.to_string()
    }

    pub fn set_headers(&self, headers: HeaderItems) -> Frame {
        Frame {
            frame_type: self.frame_type,
            headers: headers,
            body: self.body,
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
        writeln!(&mut frame, "{frame_type}", frame_type = self.frame_type);
        writeln!(&mut frame, "{}", "accept-version:1.2");
        writeln!(&mut frame, "{}", "");
        writeln!(&mut frame, "{body}\0", body = "");
        frame
    }
}
