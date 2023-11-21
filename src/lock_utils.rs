use anyhow::{anyhow, Error};
use std::sync::{Arc, RwLock};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub trait LockExt<T> {
    /// let adapters = self
    ///        .adapters
    ///        .write()
    ///        .map_err(|e| anyhow!("Failed to read-lock adapters: {}", e))?;
    ///
    /// // Is converted to:
    ///
    /// let adapters = self.adapters.lock_write("adapters")?;
    fn lock_write(&self, name: &str) -> Result<RwLockWriteGuard<T>, Error>;

    /// let adapters = self
    ///        .adapters
    ///        .read()
    ///        .map_err(|e| anyhow!("Failed to read-lock adapters: {}", e))?;
    ///
    /// // Is converted to:
    ///
    /// let adapters = self.adapters.lock_read("adapters")?;
    fn lock_read(&self, name: &str) -> Result<RwLockReadGuard<T>, Error>;
}

impl<T> LockExt<T> for Arc<RwLock<T>> {
    fn lock_write(&self, name: &str) -> Result<RwLockWriteGuard<T>, Error> {
        self.write()
            .map_err(|e| anyhow!("Failed to write-lock {}: {}", name, e))
    }

    fn lock_read(&self, name: &str) -> Result<RwLockReadGuard<T>, Error> {
        self.read()
            .map_err(|e| anyhow!("Failed to read-lock {}: {}", name, e))
    }
}
