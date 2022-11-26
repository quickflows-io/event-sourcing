# event-sourcing-rs

**STATUS** - This library is under construction and is subject to change.

## Modules

- [event-sourcing](./event-sourcing)

**Implementations:**

- [event-store-postgres](./event-store-postgres) - [PostgreSQL](https://www.postgresql.org/) implementation of the [event_store](./event-sourcing/src/envelope/store.rs) trait.
- [event-stream-kafka](./event-stream-kafka) - [Apache Kafka](https://kafka.apache.org/) implementation of the [event_stream](./event-sourcing/src/envelope/stream.rs) trait.

**Local development implementations:** (Not recommended for production)

- [event-store-inmemory](./event-store-inmemory) - [In Memory](https://www.postgresql.org/) implementation of the [event_store](./event-sourcing/src/envelope/store.rs) trait.

## Summary

An opinionated event sourcing library that is designed to be focused on speed, durability, and ease of use.

## Recommended Setup

- Add `event-sourcing-rs` to your project:
```toml
[dependencies]
event-sourcing = { git = "https://github.com/benjaminjacobberg/event-sourcing-rs" }
event-store-postgres = { git = "https://github.com/benjaminjacobberg/event-sourcing-rs" }
```
- If you are working in a microservice architecture, I recommend setting up something like Debezium up to capture the `writes` to the event table and push them to a Kafka topic.
   - Then you can stream those off the topic with:
```toml
[dependencies]
...
event-stream-kafka = { git = "https://github.com/benjaminjacobberg/event-sourcing-rs" }
```

## Architecture

**Sequence Diagram**

![sequence diagram](documentation/event-sourcing-rs-2.drawio.png?raw=true "recommended setup")

**High-level Diagram**

![high-level diagram](documentation/event-sourcing-rs.drawio.png?raw=true "recommended setup")

## Example

If you want to see an example of this library in action, please check out the [bank account example project here](examples/postgres-kafka/bank-account).

## TODO before version 1.0.0

- snapshots
- in-memory event stream
- clean up and finalize the traits
- finish the example project