# `rust_socketio` deadlock example

In version 0.6.0 of the [rust_socketio](https://github.com/1c3t3a/rust-socketio) library the method `emit_with_ack` cannot be called recursively due to a deadlock.

This repo shows how the error is achieved.

## Quick start
**Start example socketio server:**
1. `npm install`
2. `npm start`

**Run rust bin:**
1. `cargo run`

> You might hit a "String("EngineIO Error")" error. Just rerun `cargo run` and it should be working.

## Outcome
**Intended:**
- The server should log the following:
```
{ event_with_ack: 'first call' }
{ event_with_ack: 'second call' }
```

- The rust program should log:
```
Ack 1 received: Text(
    [
        Array [
            Object {
                "event_with_ack": String("first call"),
            },
        ],
    ],
)
Ack 2 received: Text(
    [
        Array [
            Object {
                "event_with_ack": String("second call"),
            },
        ],
    ],
)
```

**Actual:**
- The server only receives the first call:
```
{ event_with_ack: 'first call' }
```

- The rust program never executes the second callback and is in a deadlock:
```
Ack 1 received: Text(
    [
        Array [
            Object {
                "event_with_ack": String("first call"),
            },
        ],
    ],
)
```

## Fix
If you change the dependency in `Cargo.toml` to point to my fork and run the example again, it should work:
```toml
[dependencies]
# rust_socketio = { version = "^0.6.0" }
rust_socketio = { git = "https://github.com/bastilimbach/rust-socketio.git" }
```
