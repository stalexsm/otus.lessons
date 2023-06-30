use crate::device::{Device, SmartSocket, SmartThermometer};
use crate::room::Room;
use std::collections::HashMap;

#[derive(Clone)]
pub struct SmartHome {
    pub name: String,
    pub rooms: HashMap<String, Room>,
}

impl SmartHome {
    pub fn new(name: String) -> Self {
        SmartHome {
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn get_rooms(&self) -> Vec<&Room> {
        // Метод для попучения списка комнат
        self.rooms.values().collect()
    }

    pub fn add_room(&mut self, room: Room) -> Option<Room> {
        self.rooms.insert(room.name.clone(), room)
    }

    pub fn devices(&self, room: &str) -> Option<Vec<&Device>> {
        // Метод для попучения списка девайсов в комнате
        if let Some(target_room) = self.rooms.get(room) {
            let devices: Vec<&Device> = target_room.devices.values().collect();
            Some(devices)
        } else {
            None
        }
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        // Метод для формирования отчета в доме по комнатам и устройствам
        let mut report = String::new();

        report.push_str(&format!("Home: {}\n", &self.name));
        for room in &self.get_rooms() {
            report.push_str(&format!("Room: {}\n", room.name));
            if let Some(devices) = self.devices(&room.name) {
                for device in devices.iter() {
                    if let Some(device_info) = provider.get_device_info(&room.name, &device.name) {
                        report.push_str(&format!("Device: {}\n", device.name));
                        report.push_str(&format!("State: {}\n", device_info));
                    } else {
                        report.push_str(&format!("Device not found: {}\n", device.name));
                    }
                }
                report.push('\n');
            }
        }

        report
    }
}

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, device: &str) -> Option<String>;
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, _room: &str, device: &str) -> Option<String> {
        match device {
            "socket" => Some(self.socket.get_state()),
            _ => None,
        }
    }
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_info(&self, _room: &str, device: &str) -> Option<String> {
        match device {
            "socket" => Some(self.socket.get_state()),
            "thermometer" => Some(self.thermo.get_temperature()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_smarthome() {
        let house = SmartHome::new(String::from("My Dom"));

        assert_eq!(house.name, "My Dom");
    }

    #[test]
    fn add_room_in_smarthome() {
        let room = Room::new(String::from("My Room"));

        let mut house = SmartHome::new(String::from("My Dom"));
        house.add_room(room);

        assert_eq!(house.rooms.len(), 1);
    }

    #[test]
    fn smarthome_get_rooms() {
        let room = Room::new(String::from("My Room"));

        let mut house = SmartHome::new(String::from("My Dom"));
        house.add_room(room);

        assert_eq!(house.get_rooms().len(), 1);
    }

    #[test]
    fn smarthome_room_get_devices() {
        let device = Device::new(String::from("Socket"));
        let mut room = Room::new(String::from("My Room"));

        room.add_device(device);

        let mut house = SmartHome::new(String::from("My Dom"));
        house.add_room(room.clone());

        let devices = house.devices(&room.name).unwrap();

        assert_eq!(devices.len(), 1);
    }

    #[test]
    fn smarthome_create_report_owning() {
        let device = Device::new(String::from("Socket"));
        let mut room = Room::new(String::from("My Room"));

        room.add_device(device);

        let mut house = SmartHome::new(String::from("My Dom"));
        house.add_room(room.clone());

        let socket = SmartSocket {};
        let info_provider = OwningDeviceInfoProvider { socket };

        let report = house.create_report(&info_provider);

        assert!(report.len() > 0)
    }

    #[test]
    fn smarthome_create_report_borrowing() {
        let device1 = Device::new(String::from("socket"));
        let device2 = Device::new(String::from("thermometer"));

        let mut room = Room::new(String::from("My Room"));

        room.add_device(device1);
        room.add_device(device2);

        let mut house = SmartHome::new(String::from("My Dom"));
        house.add_room(room.clone());

        let socket = SmartSocket {};
        let thermo = SmartThermometer {};

        let info_provider = BorrowingDeviceInfoProvider {
            socket: &socket,
            thermo: &thermo,
        };

        let report = house.create_report(&info_provider);

        assert!(report.len() > 0)
    }
}
