use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

const EPOCH: u64 = 1609459200000000; // 微秒级时间戳: 2021-01-01 00:00:00 UTC
const WORKER_ID_BITS: u8 = 5;
const DATACENTER_ID_BITS: u8 = 5;
const SEQUENCE_BITS: u8 = 12;

const MAX_WORKER_ID: u64 = (1 << WORKER_ID_BITS) - 1;
const MAX_DATACENTER_ID: u64 = (1 << DATACENTER_ID_BITS) - 1;

const WORKER_ID_SHIFT: u8 = SEQUENCE_BITS;
const DATACENTER_ID_SHIFT: u8 = SEQUENCE_BITS + WORKER_ID_BITS;
const TIMESTAMP_SHIFT: u8 = SEQUENCE_BITS + WORKER_ID_BITS + DATACENTER_ID_BITS;

#[derive(Debug)]
struct Inner {
    sequence: u64,
    last_timestamp: u64,
}

#[derive(Clone)]
pub struct SnowflakeId {
    worker_id: u64,
    datacenter_id: u64,
    inner: Arc<Mutex<Inner>>,
}

impl SnowflakeId {
    pub fn new(worker_id: u64, datacenter_id: u64) -> Self {
        assert!(worker_id <= MAX_WORKER_ID);
        assert!(datacenter_id <= MAX_DATACENTER_ID);

        Self {
            worker_id,
            datacenter_id,
            inner: Arc::new(Mutex::new(Inner {
                sequence: 0,
                last_timestamp: 0,
            })),
        }
    }

    pub fn gen_id(&self) -> u64 {
        let mut inner = self.inner.lock().unwrap();
        let mut timestamp = Self::current_time();

        if timestamp == inner.last_timestamp {
            inner.sequence = (inner.sequence + 1) & ((1 << SEQUENCE_BITS) - 1);
            if inner.sequence == 0 {
                timestamp = Self::til_next_micros(inner.last_timestamp);
            }
        } else {
            inner.sequence = 0;
        }

        inner.last_timestamp = timestamp;

        ((timestamp - EPOCH) << TIMESTAMP_SHIFT)
            | (self.datacenter_id << DATACENTER_ID_SHIFT)
            | (self.worker_id << WORKER_ID_SHIFT)
            | inner.sequence
    }

    fn current_time() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64
    }

    fn til_next_micros(last_ts: u64) -> u64 {
        let mut ts = Self::current_time();
        while ts <= last_ts {
            ts = Self::current_time();
        }
        ts
    }
}
