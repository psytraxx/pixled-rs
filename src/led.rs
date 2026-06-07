use anyhow::{Context, Result};
use ddp_rs::{connection, protocol};
use std::net::UdpSocket;

pub struct LedOutput {
    conn: connection::DDPConnection,
}

impl LedOutput {
    pub fn try_new(host: &str) -> Result<Self> {
        let conn = connection::DDPConnection::try_new(
            host,
            protocol::PixelConfig::default(),
            protocol::ID::Default,
            UdpSocket::bind("0.0.0.0:4048").context("Failed to bind UDP socket")?,
        )
        .context("Failed to connect to DDP host")?;
        Ok(Self { conn })
    }

    pub fn send(&mut self, pixel_values: &[u8]) -> Result<()> {
        self.conn
            .write(pixel_values)
            .context("Failed to send DDP frame")?;
        Ok(())
    }
}
