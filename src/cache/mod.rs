use std::time::{Duration, Instant};

use crate::{
    tree::Tree,
    types::{dns::Name, rr::Record},
};

mod status;

pub use status::*;

pub struct Cache {
    inner: Tree<CachedRecord>,
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
                .checked_add(Duration::from_secs(record.get_header().ttl.into()))
                .unwrap();

            cached_records.push(CachedRecord { expires_at, record })
        }

        self.inner.insert(name, &mut cached_records);
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
