use web_sys::Storage;

/// 本地存储工具类
pub struct LocalStorage;

impl LocalStorage {
    /// 获取本地存储对象
    fn get_storage() -> Option<Storage> {
        web_sys::window()?.local_storage().ok().flatten()
    }

    /// 存储数据到本地存储
    pub fn set<T: serde::Serialize>(key: &str, value: &T) -> Result<(), String> {
        let storage = Self::get_storage().ok_or("无法获取本地存储")?;
        let json = serde_json::to_string(value).map_err(|e| e.to_string())?;
        storage
            .set_item(key, &json)
            .map_err(|e| format!("存储失败: {:?}", e))?;
        Ok(())
    }

    /// 从本地存储获取数据
    pub fn get<T: for<'de> serde::Deserialize<'de>>(key: &str) -> Result<T, String> {
        let storage = Self::get_storage().ok_or("无法获取本地存储")?;
        let json = storage
            .get_item(key)
            .map_err(|e| format!("读取失败: {:?}", e))?;
        let json = json.ok_or("未找到数据")?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }

    /// 从本地存储移除数据
    pub fn remove(key: &str) -> Result<(), String> {
        let storage = Self::get_storage().ok_or("无法获取本地存储")?;
        storage
            .remove_item(key)
            .map_err(|e| format!("删除失败: {:?}", e))?;
        Ok(())
    }

    /// 清空本地存储
    pub fn clear() -> Result<(), String> {
        let storage = Self::get_storage().ok_or("无法获取本地存储")?;
        storage.clear().map_err(|e| format!("清空失败: {:?}", e))?;
        Ok(())
    }
}
