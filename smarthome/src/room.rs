use super::device::Device;
use std::collections::HashMap;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Clone)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub devices: HashMap<Uuid, Device>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device: Device) {
        if let Some(id) = device.get_id() {
            self.devices.insert(id, device);
        }
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}\nRoom: {}", &self.id, &self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::device::SmartSocket;

    use super::*;

    #[test]
    fn create_room() {
        let room = Room::new(String::from("My Room"));

        assert_eq!(room.name, "My Room");
    }

    #[test]
    fn add_devicein_room() {
        let socket = SmartSocket::new(String::from("My Socket"));

        let mut room = Room::new(String::from("My Room"));
        room.add_device(Device::Socket(socket));

        assert_eq!(room.devices.len(), 1);
    }
}
