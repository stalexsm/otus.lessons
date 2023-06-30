use super::device::Device;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Room {
    pub name: String,
    pub devices: HashMap<String, Device>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Room {
            name,
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device: Device) -> Option<Device> {
        self.devices.insert(device.name.clone(), device)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_room() {
        let room = Room::new(String::from("My Room"));

        assert_eq!(room.name, "My Room");
    }

    #[test]
    fn add_devicein_room() {
        let device = Device::new(String::from("My Device"));

        let mut room = Room::new(String::from("My Room"));
        room.add_device(device);

        assert_eq!(room.devices.len(), 1);
    }
}
