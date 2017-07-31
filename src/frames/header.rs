use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct HeaderItems {
    headers: Vec<String>,
}

impl Clone for HeaderItems {
    fn clone(&self) -> HeaderItems {
        HeaderItems { headers: self.headers.clone() }
    }
}

impl Display for HeaderItems {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for header in self.headers.clone() {
            writeln!(f, "{}", header)?;
        }

        Ok(())
    }
}

impl HeaderItems {
    pub fn new() -> HeaderItems {
        HeaderItems { headers: Vec::new() }
    }

    pub fn add_accept_version(self, version: &'static str) -> HeaderItems {
        self.add_header("accept-version", version)
    }

    pub fn add_accept_version_default(self) -> HeaderItems {
        self.add_accept_version("1.2")
    }

    pub fn add_host(self, host: &'static str) -> HeaderItems {
        self.add_header("host", host)
    }

    pub fn add_login(self, user: &'static str, passwd: &'static str) -> HeaderItems {
        self.add_header("login", user)
            .add_header("passcode", passwd)
    }

    fn add_header(mut self, key: &'static str, value: &'static str) -> HeaderItems {
        let mut header: String = String::new();
        header.push_str(key);
        header.push(':');
        header.push_str(value);
        self.headers.push(header);
        self
    }
}
