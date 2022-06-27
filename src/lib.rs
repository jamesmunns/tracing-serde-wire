//! # Wire types for use with tracing-serde-structured crate
//!
//! This crate contains a number of high level types that contain the individual
//! [tracing-rs] event types that are created via the [tracing-serde-structured]
//! crate.
//!
//! These types can be used as a simple "wire format" between a tracing subscriber
//! which obtains and serializes the events, and a remote collector, which receives
//! and deserializes the events
//!
//! [tracing-rs]: https://tracing.rs
//! [tracing-serde-structured]: https://github.com/jamesmunns/tracing-serde-structured
//!
//! Currently, this crate contains no code, and only data type definitions.
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![no_std]

use serde::{Serialize, Deserialize};

use tracing_serde_structured::{
    SerializeAttributes,
    SerializeRecord,
    SerializeEvent,
    SerializeId,
};

/// Tracing Wire "Other" messages
///
/// This type contains messages that are not directly sourced from tracing events,
/// but are sometimes necessary during establishing or maintaining of the connection.
#[derive(Debug, Serialize, Deserialize)]
pub enum TWOther {
    /// One or more events have been discarded without being transmitted via a `Packet`.
    /// This is used when the local buffer has become full, and it was no longer possible
    /// to transmit the stream of events uninterrupted.
    MessageDiscarded,

    /// A collection of metadata items about the device that is producing trace events.
    DeviceInfo {
        clock_id: u32,
        ticks_per_sec: u32,
        device_id: [u8; 16],
    },
}

/// A tracing wire Packet type
///
/// This type is the "outer wrapper", typically sent as the single message on the wire.
/// Devices producing tracing messages will emit a stream of Packets, which will be
/// individually processed by the remote collector
#[derive(Debug, Serialize, Deserialize)]
pub struct Packet<'a> {
    #[serde(borrow)]
    pub message: TracingWire<'a>,
    pub tick: u64,
}

/// The tracing wire event enumeration
///
/// This enumeration contains all possible event kinds that are produced by tracing-rs,
/// plus a `TWOther` variant for non-tracing metadata to be sent.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TracingWire<'a> {
    NewSpan {
        id: SerializeId,
        #[serde(borrow)]
        attrs: SerializeAttributes<'a>,
        values: SerializeRecord<'a>,
    },
    Record {
        values: SerializeRecord<'a>,
        span: SerializeId,
    },
    RecordFollowsFrom {
        follows: SerializeId,
        span: SerializeId,
    },
    Event(SerializeEvent<'a>),
    Enter(SerializeId),
    Exit(SerializeId),
    Close(SerializeId),
    IdClone {
        new: SerializeId,
        old: SerializeId,
    },
    Other(TWOther),
}
