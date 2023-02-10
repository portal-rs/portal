use binbuf::prelude::*;

#[derive(Debug, Clone)]
pub struct COOKIE {
    client: Vec<u8>,
    server: Option<Vec<u8>>,
}

impl COOKIE {
    pub fn read<E: Endianness>(buf: &mut ReadBuffer, len: u16) -> Result<Self, BufferError> {
        // If the len is only 8 octets, we know that only the client cookie is
        // present, so we take the short path
        if len == 8 {
            let client = buf.read_vec(8)?;
            return Ok(Self {
                client,
                server: None,
            });
        }

        // Len is longer than 8 octets, both client and server cookie are
        // present
        let client = buf.read_vec(8)?;
        let server = buf.read_vec((len - 8) as usize)?;

        Ok(Self {
            client,
            server: Some(server),
        })
    }
}

impl Writeable for COOKIE {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let mut n = buf.write(&self.client);

        if let Some(server) = &self.server {
            n *= buf.write(server);
        }

        Ok(n)
    }
}
