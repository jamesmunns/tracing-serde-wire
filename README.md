# Wire types for use with tracing-serde-structured crate

This crate contains a number of high level types that contain the individual
[tracing-rs] event types that are created via the [tracing-serde-structured]
crate.

These types can be used as a simple "wire format" between a tracing subscriber
which obtains and serializes the events, and a remote collector, which receives
and deserializes the events

[tracing-rs]: https://tracing.rs
[tracing-serde-structured]: https://github.com/jamesmunns/tracing-serde-structured

Currently, this crate contains no code, and only data type definitions.

## License

This crate is provided under the terms of the [Apache 2.0 license].

[Apache 2.0 license]: ./LICENSE
