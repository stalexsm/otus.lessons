use smarthome::device::{Device, SmartSocket, SmartThermometer};
use smarthome::home::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider, SmartHome};
use smarthome::room::Room;

fn main() {
    let socket1 = SmartSocket::new(String::from("Socket 1"));
    let mut socket2 = SmartSocket::new(String::from("Socket 2"));
    let thermo = SmartThermometer::new(String::from("Thermometer 1"), 18.8);

    // Инициализация дома
    let mut living_room = Room::new(String::from("Living Room"));

    living_room.add_device(Device::Socket(socket1.clone()));

    let mut kitchen = Room::new(String::from("Kitchen"));

    socket2.switch(true);

    kitchen.add_device(Device::Socket(socket2.clone()));
    kitchen.add_device(Device::Thermometer(thermo.clone()));

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
    println!("Report #1:\n{report1}");
    println!("Report #2:\n{report2}");
}
