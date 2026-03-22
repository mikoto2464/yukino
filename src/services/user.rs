use crate::error::YukinoError;

struct Device {
    name: String,
    hardware_id: String,
}

pub async fn devices(user_id: u32) -> Result<[Device], YukinoError> {
    
}