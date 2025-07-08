use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

pub struct LocalStorageService;

impl LocalStorageService {
    pub fn set<T>(key: &str, value: &T) -> Result<(), gloo_storage::errors::StorageError>
    where
        T: Serialize,
    {
        LocalStorage::set(key, value)
    }

    pub fn get<T>(key: &str) -> Result<T, gloo_storage::errors::StorageError>
    where
        T: for<'de> Deserialize<'de>,
    {
        LocalStorage::get(key)
    }

    pub fn remove(key: &str) {
        LocalStorage::delete(key);
    }

    pub fn clear() {
        LocalStorage::clear();
    }
}
