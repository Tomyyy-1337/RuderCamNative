use nsh_core::storage::Storage;

/// A trait for persistent stores backed by `nsh_core::storage::Storage`.
/// The `KEY constant must be unique for each implementor and is used as the storage key in `nsh_core::storage::Storage`.
pub trait Store 
where 
    Self: serde::Serialize + serde::de::DeserializeOwned
{
    const KEY: &'static str;

    /// Load from persistent storage.
    /// Returns `None` if the store doesn't exist or cannot be deserialized.
    fn load() -> Option<Self> {
        Storage::get_item(Self::KEY)
            .and_then(|raw| serde_json::from_str(&raw).ok())
    }

    // Safe to persistant storage.
    fn save(&self) {
        if let Ok(raw) = serde_json::to_string(self) {
            if let Err(error) = Storage::set_item(Self::KEY, raw) {
                eprintln!("store: {error}");
            }
        }
    }
}