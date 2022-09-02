# event-sourcing-rs

**STATUS** - This library is under construction and is subject to change.  

## Modules

- [event-sourcing](./event-sourcing)
- [event-store-inmemory](./event-store-inmemory) - [In Memory](https://www.postgresql.org/) implementation of the [event_store](event-sourcing/src/envelope/store.rs) trait.
- [event-store-postgres](./event-store-postgres) - [PostgreSQL](https://www.postgresql.org/) implementation of the [event_store](event-sourcing/src/envelope/store.rs) trait.
- (TODO) [event-stream-inmemory](./event-stream-inmemory) - [In Memory](https://www.postgresql.org/) implementation of the [event_stream](event-sourcing/src/envelope/stream.rs) trait.
- [event-stream-kafka](./event-stream-kafka) - [Apache Kafka](https://kafka.apache.org/) implementation of the [event_stream](event-sourcing/src/envelope/stream.rs) trait.

## Summary

An opinionated event sourcing library that is designed to be focused on speed, durability, and ease of use.

### Example (WIP)

If you want to see an example of this library in action, please check out the [bank account example project here](examples/postgres-kafka/bank-account).

### Architecture

![event sourcing architecture](documentation/event-sourcing-rs.drawio.png?raw=true "event sourcing architecture")

### TODO before version 1.0.0

- snapshots
- in-memory event stream
- clean up and finalize the traits
- finish the example project