# Portal

Portal is a DNS server written in pure Rust. This project has two purposes:

- Challenge me to write more Rust code.
- Extend the features of the initial Go implementation.

Sometime in the future, this maybe will be production ready.

## TODOs

- Move Config code to portal-bin
- Split code into multiple crates
- Make handlers more flexible (Generic)
- Implement a DNS multiplexer to better handle requests, responses, and transaction ids
- Split zone file loading into Lexer and Parser