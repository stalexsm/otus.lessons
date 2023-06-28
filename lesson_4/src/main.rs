fn main() {
    todo!()
}

// Состояние розетки
enum _SocketCondition {
    On,
    Off,
}

// Структура, представляющая умную розетку
struct _SmartSocket {
    condition: _SocketCondition,
    power_consumption: f32,
    description: String,
}

impl _SmartSocket {
    fn _new(desc: String) -> Self {
        // Инициализация объекта
        _SmartSocket {
            condition: _SocketCondition::Off,
            power_consumption: 0.0,
            description: desc,
        }
    }

    fn _turn(&mut self, cond: _SocketCondition) {
        // Включение  и выключение розетки
        self.condition = cond
    }

    fn _get_power_consumption(&self) -> f32 {
        // получение данных о потребляемой мощности
        self.power_consumption
    }

    fn _set_power_consumption(&mut self, power: f32) {
        // установка данных о потребляемой мощности
        self.power_consumption = power
    }

    fn _get_description(&self) -> &String {
        // Предоставлять текстовое описание
        &self.description
    }
}

// Структура, представляющая термометр
struct _Thermometer {
    temperature: f32,
}

impl _Thermometer {
    fn _new() -> Self {
        // Инициализация объекта
        _Thermometer { temperature: 0.0 }
    }

    fn _get_temperature(&self) -> f32 {
        // Получение данные о текущей температуре
        self.temperature
    }

    fn _set_temperature(&mut self, temperature: f32) {
        // Изменение данные о текущей температуре
        self.temperature = temperature;
    }
}
