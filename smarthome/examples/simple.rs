use smarthome::device::{Device, SmartSocket, SmartThermometer};
use smarthome::home::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider, SmartHome};
use smarthome::room::Room;

fn main() {
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};

    // Инициализация дома
    let mut living_room = Room::new(String::from("Living Room"));

    living_room.add_device(Device::new(String::from("TV")));
    living_room.add_device(Device::new(String::from("socket")));

    let mut kitchen = Room::new(String::from("Kitchen"));
    kitchen.add_device(Device::new(String::from("Refrigerator")));
    kitchen.add_device(Device::new(String::from("thermometer")));

    // Инициализация дома
    let mut house = SmartHome::new(String::from("My Dom"));

    house.add_room(living_room);
    house.add_room(kitchen);

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
