pub struct Service {
    count: std::sync::RwLock<u64>,
    count_thread1: std::sync::RwLock<u64>,
    count_thread2: std::sync::RwLock<u64>,
    write_lock: std::sync::RwLock<u8>,
}

impl Service {
    pub fn new() -> Self {
        Service {
            count: std::sync::RwLock::new(0),
            count_thread1: std::sync::RwLock::new(0),
            count_thread2: std::sync::RwLock::new(0),
            write_lock: std::sync::RwLock::new(0),
        }
    }

    pub fn get_counts(&self) -> Result<(u64, u64, u64), String> {
        let count = *self
            .count
            .read()
            .map_err(|e| format!("Failed to read-lock count: {}", e))?;
        let count_thread1 = *self
            .count_thread1
            .read()
            .map_err(|e| format!("Failed to read-lock count_thread1: {}", e))?;
        let count_thread2 = *self
            .count_thread2
            .read()
            .map_err(|e| format!("Failed to read-lock write_lock: {}", e))?;
        Ok((count, count_thread1, count_thread2))
    }

    pub fn increment_counts_thread1(&self) -> Result<(u64, u64, u64), String> {
        let mut count = self
            .count
            .write()
            .map_err(|e| format!("Failed to write-lock count: {}", e))?;
        let mut count_thread1 = self
            .count_thread1
            .write()
            .map_err(|e| format!("Failed to write-lock count_thread1: {}", e))?;
        let mut write_lock = self
            .write_lock
            .write()
            .map_err(|e| format!("Failed to write-lock write_lock: {}", e))?;
        *count += 1;
        *count_thread1 += 1;
        *write_lock = 1;
        drop(count);
        drop(count_thread1);
        self.get_counts()
    }

    pub fn increment_counts_thread2(&self) -> Result<(u64, u64, u64), String> {
        let mut count = self
            .count
            .write()
            .map_err(|e| format!("Failed to write-lock count: {}", e))?;
        let mut count_thread2 = self
            .count_thread2
            .write()
            .map_err(|e| format!("Failed to write-lock count_thread2: {}", e))?;
        let mut write_lock = self
            .write_lock
            .write()
            .map_err(|e| format!("Failed to write-lock write_lock: {}", e))?;
        *count += 1;
        *count_thread2 += 1;
        drop(count);
        drop(count_thread2);
        *write_lock = 2;
        self.get_counts()
    }
}
