mod classes;
mod types;

pub use classes::*;
pub use types::*;

pub mod a;
pub mod aaaa;

pub trait ResourceRecord: ToString + PartialEq<Self> {
    fn header(&self) -> Header;
    fn set_header(&self, header: Header);
    fn len(&self) -> u16;
    fn unpack(&self, data: Vec<u8>, offset: usize) -> Result<usize, ()>;
    fn pack(&self, buf: Vec<u8>, offset: usize) -> Result<usize, ()>;
}

#[derive(Debug)]
pub struct Header {
    name: String,
    typ: Type,
    class: Class,
    ttl: u32,
    rdlen: u16,
}

#[derive(Debug)]
pub enum RR {
    A(a::A),
    AAAA(aaaa::AAAA),
}
