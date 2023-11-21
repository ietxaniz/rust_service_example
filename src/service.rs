use crate::lock_utils::LockExt;
use anyhow::Error;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Service {
    count: Arc<RwLock<u64>>,
    count_thread1: Arc<RwLock<u64>>,
    count_thread2: Arc<RwLock<u64>>,
    write_lock: Arc<RwLock<u8>>,
}

impl Service {
    pub fn new() -> Self {
        Service {
            count: Arc::new(RwLock::new(0)),
            count_thread1: Arc::new(RwLock::new(0)),
            count_thread2: Arc::new(RwLock::new(0)),
            write_lock: Arc::new(RwLock::new(0)),
        }
    }

    pub fn get_counts(&self) -> Result<(u64, u64, u64), Error> {
        let count = *self.count.lock_read("count")?;
        let count_thread1 = *self.count_thread1.lock_read("count_thread1")?;
        let count_thread2 = *self.count_thread2.lock_read("count_thread2")?;
        Ok((count, count_thread1, count_thread2))
    }

    pub fn increment_counts_thread1(&self) -> Result<(u64, u64, u64), Error> {
        let mut count = self.count.lock_write("count")?;
        let mut count_thread1 = self.count_thread1.lock_write("count_thread1")?;
        let mut write_lock = self.write_lock.lock_write("write_lock")?;
        *count += 1;
        *count_thread1 += 1;
        *write_lock = 1;
        drop(count);
        drop(count_thread1);
        self.get_counts()
    }

    pub fn increment_counts_thread2(&self) -> Result<(u64, u64, u64), Error> {
        let mut count = self.count.lock_write("count")?;
        let mut count_thread2 = self.count_thread2.lock_write("count_thread2")?;
        let mut write_lock = self.write_lock.lock_write("write_lock")?;
        *count += 1;
        *count_thread2 += 1;
        drop(count);
        drop(count_thread2);
        *write_lock = 2;
        self.get_counts()
    }
}
