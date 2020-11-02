# Testable Rust Web Project

An example project that implements Ports-and-Adapters pattern. This pattern helps with testability of the project.

Read the accompanying article that explains this repository: [Structuring Rust Project For Testability](https://eckyputrady.medium.com/structuring-rust-project-for-testability-18207b5d0243).

## Running The Test

Setup the infrastructure required for testing via Docker:

```
$ sudo docker stack deploy -c test-stack.yml testable-rust-web-project
```

Run the test:

```
$ cargo test
```

## Running The Application

Setup the infrastructure required for running the application via Docker:

```
$ sudo docker stack deploy -c test-stack.yml testable-rust-web-project
```

Run the application:

```
$ cargo run
```

Interact with the application using `curl` or other HTTP testing tools.
