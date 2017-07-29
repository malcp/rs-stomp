#[derive(Debug)]
pub struct HeaderItems {
    pub headers: Vec<String>,
}

impl Clone for HeaderItems {
    fn clone(&self) -> HeaderItems {
        HeaderItems { headers: self.headers.clone() }
    }
}
