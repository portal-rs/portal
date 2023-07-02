# portal

⚠️ This implementation is currently not production ready. However, it achieves to be in the future!

Portal is a DNS implementation written in pure Rust. It provides multiple crates and binaries to work with the DNS
protocol. The protocol implementation is located in these crates:

- `crates/client`: A sequential and multiplexed DNS client implementation
- `crates/common`: A collection of common data types and utility functions used in other crates
- `crates/proto`: The base protocol implementation
- `crates/resolver`: Forwarding and recursive resolver implementations
- `crates/server`: A DNS server implementation

## TODOs

- Move Config code to portal-bin
- Make handlers more flexible (Generic)
- Implement a DNS multiplexer to better handle requests, responses, and transaction ids
- Split zone file loading into Lexer and Parser