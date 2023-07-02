# portal-client

This crate provides a full-featured DNS client implementation. Currently, the crate exposes two clients: a simple client
for sequential message exchanges and a multiplexed client, which allows concurrent receiving and sending of messages.
The two clients are:

- `portal_client::Client`
- `portal_client::MultiplexClient`

pgun, the **p**ortable **g**roper **u**tility for **n**ameservers, a fast terminal-based DNS query tool, uses the
multiplexed client to enable advanced features like sending query messages in parallel. In addition, both the recursive
and the forwarding resolvers use the multiplexed client to achieve maximum performance.