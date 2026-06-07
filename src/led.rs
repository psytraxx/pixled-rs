use ddp_rs::{connection, protocol};
use std::{error::Error, net::UdpSocket};

pub struct LedOutput {
    conn: connection::DDPConnection,
}

impl LedOutput {
    pub fn try_new(host: &str) -> Result<Self, Box<dyn Error>> {
        let conn = connection::DDPConnection::try_new(
            host,
            protocol::PixelConfig::default(),
            protocol::ID::Default,
            UdpSocket::bind("0.0.0.0:4048")?,
        )?;
        Ok(Self { conn })
    }

    pub fn send(&mut self, pixel_values: &[u8]) -> Result<(), Box<dyn Error>> {
        self.conn.write(pixel_values)?;
        Ok(())
    }
}
