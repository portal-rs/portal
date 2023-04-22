use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use portal_proto::{Name, RType, Record};

mod status;

pub use status::*;

// FIXME (Techassi): This idealy should be a tree I guess. But I'm not quite sure if the tree is faster. Also is a
// hashmap the fastest solution? This cache also naively does NOT store any expiration timestamp. Once a record is
// stored, it will never be invalidated. The current implementation is only a placeholder and is only in place so
// that the server is comparable to other DNS servers (with caching).
pub struct Cache {
    inner: HashMap<Name, HashMap<RType, Record>>,
    // inner: Tree,
}

impl Cache {
    pub fn insert(&mut self, name: Name, records: Vec<Record>) {
        let mut cached_records: Vec<CachedRecord> = Vec::new();

        // NOTE (Techassi): This can introduce inaccurate expire timestamps,
        // but we avoid calling Instant::now() on every added record.
        let now = Instant::now();

        for record in records {
            // TODO (Techassi): Handle this error
            let expires_at = now
                .checked_add(Duration::from_secs(record.header().ttl().into()))
                .unwrap();

            cached_records.push(CachedRecord { expires_at, record })
        }

        // self.inner.insert_multi(name, &mut cached_records);
    }
}

pub struct CachedRecord {
    expires_at: Instant,
    record: Record,
}

impl CachedRecord {
    pub fn get_expires_at(&self) -> Instant {
        self.expires_at
    }

    pub fn get_record(&self) -> &Record {
        &self.record
    }
}
