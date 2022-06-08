#![no_std]

use serde::{Serialize, Deserialize};

use tracing_serde_structured::{
    SerializeAttributes,
    SerializeRecord,
    SerializeEvent,
    SerializeId,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum TWOther {
    MessageDiscarded,
    DeviceInfo {
        clock_id: u32,
        ticks_per_sec: u32,
        device_id: [u8; 16],
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet<'a> {
    #[serde(borrow)]
    pub message: TracingWire<'a>,
    pub tick: u64,
}

#[derive(Debug, Serialize, Deserialize)]
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
    Other(TWOther),
}
