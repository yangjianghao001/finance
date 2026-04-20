use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    port: Option<u16>,
}

impl Server {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3000)
    }

    pub fn ip_addr(&self) -> String {
        local_ip_addr::get_local_ip_address().unwrap_or("127.0.0.1".into())
    }
}
