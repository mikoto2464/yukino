use crate::error::YukinoError;

struct DevicesRes {
    name: String,
    hardware_id: String,
    
}

pub async fn devices() -> Result<[DevicesRes], YukinoError>