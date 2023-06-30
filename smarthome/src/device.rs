#[derive(Clone)]
pub struct Device {
    pub name: String,
}

impl Device {
    pub fn new(name: String) -> Self {
        Device { name }
    }
}

pub struct SmartSocket {}
impl SmartSocket {
    pub fn get_state(&self) -> String {
        String::from("On")
    }
}

pub struct SmartThermometer {}
impl SmartThermometer {
    pub fn get_temperature(&self) -> String {
        String::from("25°C")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_device() {
        let device = Device::new(String::from("My Device"));

        assert_eq!(device.name, "My Device");
    }

    #[test]
    fn get_temperature_in_thermo() {
        let device = SmartThermometer {};

        assert_eq!(device.get_temperature(), "25°C");
    }

    #[test]
    fn get_state_in_socket() {
        let device = SmartSocket {};

        assert_eq!(device.get_state(), "On");
    }
}
