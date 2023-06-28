//  Структура умного дома
struct SmartHouse {
    name: String,
    rooms: Vec<Room>,
}

//  Структура Комнаты
struct Room {
    name: String,
    devices: Vec<Device>,
}

//  Структура девайса
struct Device {
    name: String,
}

impl SmartHouse {
    fn new(name: String, rooms: Vec<Room>) -> Self {
        SmartHouse { name, rooms }
    }

    fn get_rooms(&self) -> Vec<&Room> {
        // Метод для попучения списка комнат
        self.rooms.iter().collect()
    }

    fn devices(&self, room: &str) -> Option<Vec<&Device>> {
        // Метод для попучения списка девайсов в комнате
        if let Some(target_room) = self.rooms.iter().find(|r| r.name == room) {
            let devices: Vec<&Device> = target_room.devices.iter().collect();
            Some(devices)
        } else {
            None
        }
    }

    fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
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

trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, device: &str) -> Option<String>;
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
struct SmartSocket {}
impl SmartSocket {
    fn get_state(&self) -> String {
        String::from("On")
    }
}

struct SmartThermometer {}
impl SmartThermometer {
    fn get_temperature(&self) -> String {
        String::from("25°C")
    }
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, _room: &str, device: &str) -> Option<String> {
        match device {
            "socket" => Some(self.socket.get_state()),
            _ => None,
        }
    }
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
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

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};

    // Инициализация дома
    let living_room = Room {
        name: String::from("Living Room"),
        devices: vec![
            Device {
                name: String::from("TV"),
            },
            Device {
                name: String::from("socket"),
            },
        ],
    };

    let kitchen = Room {
        name: String::from("Kitchen"),
        devices: vec![
            Device {
                name: String::from("Refrigerator"),
            },
            Device {
                name: String::from("thermometer"),
            },
        ],
    };

    // Инициализация дома
    let house = SmartHouse::new(String::from("My Dom"), vec![living_room, kitchen]);

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
