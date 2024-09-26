use rust_socketio::ClientBuilder;
use serde_json::json;
use std::{thread::sleep, time::Duration};

fn main() {
    let socket = ClientBuilder::new("http://localhost:3000/")
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .connect()
        .expect("Connection failed");

    socket
        .emit_with_ack(
            "event_with_ack",
            json!({"event_with_ack": "first call"}),
            Duration::from_secs(20000),
            move |data, s| {
                println!("Ack 1 received: {:#?}", data);

                // This works
                s.emit("event", json!({"event": "second call"}))
                    .expect("Server unreachable");

                // This doesn't work due to deadlock on "outstanding_acks" in emit_with_ack()
                s.emit_with_ack(
                    "event_with_ack",
                    json!({"event_with_ack": "second call"}),
                    Duration::from_secs(20000),
                    |data, _| {
                        println!("Ack 2 received: {:#?}", data);
                    },
                )
                .expect("Server unreachable");
            },
        )
        .expect("Server unreachable");

    sleep(Duration::from_secs(200));
}
