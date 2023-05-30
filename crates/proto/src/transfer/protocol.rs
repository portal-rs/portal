#[cfg(feature = "clap")]
use clap::ValueEnum;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum Protocol {
    /// Use UDP for transport
    Udp,

    /// Use TCP for transport
    Tcp,
}
