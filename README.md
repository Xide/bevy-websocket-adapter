# Bevy websocket

> Simple adapter to receive WebSocket messages in your [bevy](https://bevyengine.org/) games as native Rust types.

<hr>

:construction: | This is a work in progress, many major features are not implemented
:---: | :---

It uses `tokio` as the async backend and `tungstenite` for websocket protocol implementation.

You can check the [`examples`](./examples) directory for more details on the usage of this crate.

<hr>

### Table of content

- [Message format](#message-format)
- [TODO](#TODO)


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


#### TODO



- [x] Receive message from clients
- [x] Deserialize messages to native Rust types
- [ ] Send messages to clients
- [ ] Broadcast message
- [ ] Client
- [ ] Raw message EventReader in Bevy
- [ ] Connect / Disconnect EventReader
- [ ] Unmatched messages EventReader
