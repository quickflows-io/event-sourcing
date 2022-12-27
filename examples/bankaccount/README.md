# Bank Account (Example Project)

## Summary

A simple REST API to demonstrate how to utilize the library.

## Getting Started

- Download the Scylla CDC Source Connector Jar:
```shell
(cd ./docker/connect/plugins ; sh download-scylla-cdc-source-connector-jar.sh)
```

- Use docker-compose to startup kafka and scylla:
```shell
(cd ./docker/ ; docker-compose up)
```

- Run the service with:
```shell
cargo run
```

- Configure the Kafka Connector to read Scylla's CDC table.  Example in: [connector.http](./connector.http)

- Instructions on how to interact with the API located in [api.http](./api.http).
