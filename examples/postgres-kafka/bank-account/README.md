# bank-account (postgres-kafka)

## Summary

A local Bank Account example project to demonstrate the functionality of this event-sourcing-rs library.

## Getting Started

- Rule the docker-compose file: [docker-compose](https://github.com/docker/compose): `$ (cd deployment; docker-compose up -d)` 
- Now start the web service: `$ cargo run`
- Use the [HTTP Client file](bank_account.http) to test out the web service.

## Project Layout

### API

Provides a REST API for executing tasks in the application layer.

### Application

Defines the jobs the software is supposed to do and directs the expressive domain objects to work out problems.

### Domain

Responsible for representing concepts of the business, information about the business situation, and business rules.

### Infrastructure

Accesses external services such as database, messaging systems and email services.

## TODO

- Flesh out the project with an API Gateway and Identity Provider.