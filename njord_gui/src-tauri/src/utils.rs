use std::time::Duration;
use njord_backend::device::Device;

pub async fn ping_and_reconnect(device: &mut Device) -> Result<(), String>{
    let ping = device
        .test_connection(Duration::from_millis(800), Duration::from_millis(150))
        .await;
    if !ping {
        device.open_connection()?;
        if !device
            .test_connection(Duration::from_millis(800), Duration::from_millis(150))
            .await
        {
            return Err("Failed connecting device after setting config".to_string());
        }
    }
    Ok(())
}