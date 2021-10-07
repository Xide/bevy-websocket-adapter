# Bevy websocket adapter

> Simple adapter to receive WebSocket messages in your [bevy](https://bevyengine.org/) games as native Rust types.


<p align="center">
    <a href="https://crates.io/crates/bevy_websocket_adapter">
        <img src="https://img.shields.io/crates/v/bevy_websocket_adapter?logo=rust" alt="crates.io">
    </a>
    <a href="https://docs.rs/bevy_websocket_adapter">
        <img src="https://docs.rs/bevy_websocket_adapter/badge.svg" alt="docs.rs">
    </a>
    <img src="https://img.shields.io/crates/l/bevy_websocket_adapter" alt="license" />
</p>


<hr>

:construction: | This is a work in progress, many major features are not implemented
:---: | :---

It uses `tokio` as the async backend and `tungstenite` for websocket protocol implementation.

You can check the [`examples`](./examples) directory for more details on the usage of this crate.

<hr>

### Table of content

- [Message format](#message-format)
- [Roadmap](#roadmap)


#### Message format

All websocket messages must be JSON object in this format:

```json
{
    "t": "MyMessageType",
    "d": "..."
}
```

:warning: | The `t` field MUST be unique across all your messages types, one value of `t` will always map to the same native rust type.
:---: | :---

The contents of `d` can be any valid JSON value. Your native rust type must be able to serialize/deserialize the contents of `d` using the `serde_json` crate.


#### Roadmap



- [x] Receive message from clients
- [x] Deserialize messages to native Rust types
- [x] Connect / Disconnect EventReader
- [ ] Send messages to clients
- [ ] Broadcast message
- [ ] Client
- [ ] Raw message EventReader in Bevy
- [ ] Unmatched messages EventReader
