use crate::packing::{PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult};

#[derive(Debug, Clone)]
pub struct COOKIE {
    client: Vec<u8>,
    server: Option<Vec<u8>>,
}

impl COOKIE {
    pub fn unpack(buf: &mut UnpackBuffer, len: u16) -> UnpackBufferResult<Self> {
        // If the len is only 8 octets, we know that only the client cookie is
        // present, so we take the short path
        if len == 8 {
            let client = buf.unpack_vec(8)?;
            return Ok(Self {
                client,
                server: None,
            });
        }

        // Len is longer than 8 octets, both client and server cookie are
        // present
        let client = buf.unpack_vec(8)?;
        let server = buf.unpack_vec((len - 8) as usize)?;

        Ok(Self {
            client,
            server: Some(server),
        })
    }
}

impl Packable for COOKIE {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        buf.pack_vec(&mut self.client.clone())?;

        if let Some(server) = &self.server {
            buf.pack_vec(&mut server.clone())?;
        }

        Ok(())
    }
}
