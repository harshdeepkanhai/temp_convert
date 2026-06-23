use std::io;

#[derive(Debug)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

fn convert_temp(temp: f64, from_unit: &TemperatureUnit, to_unit: &TemperatureUnit) -> f64 {
    match (from_unit, to_unit) {
        (TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit) => temp * 9.0 / 5.0 + 32.0,
        (TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius) => (temp - 32.0) * 5.0 / 9.0,
        (TemperatureUnit::Fahrenheit, TemperatureUnit::Kelvin) => {
            ((temp - 32.0) * 5.0 / 9.0) + 273.15
        }
        (TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit) => {
            (temp - 273.15) * 9.0 / 5.0 + 32.0
        }
        (TemperatureUnit::Celsius, TemperatureUnit::Kelvin) => temp + 273.15,
        (TemperatureUnit::Kelvin, TemperatureUnit::Celsius) => temp - 273.15,
        (TemperatureUnit::Celsius, TemperatureUnit::Celsius) => temp,
        (TemperatureUnit::Fahrenheit, TemperatureUnit::Fahrenheit) => temp,
        (TemperatureUnit::Kelvin, TemperatureUnit::Kelvin) => temp,
    }
}

fn convert_to_unit(input_unit: char) -> Result<TemperatureUnit, String> {
    match input_unit {
        'C' => Ok(TemperatureUnit::Celsius),
        'F' => Ok(TemperatureUnit::Fahrenheit),
        'K' => Ok(TemperatureUnit::Kelvin),
        _ => Err("Wrong Temperature Unit Input".to_string()),
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    let parts: Vec<&str> = input.split_whitespace().collect();
    let temp = parts[0].parse::<f64>().expect("Error Parsing Temperature");
    let from = match convert_to_unit(parts[1].parse::<char>().expect("Error Parsing Unit")) {
        Ok(input_unit) => input_unit,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };
    let to: TemperatureUnit =
        match convert_to_unit(parts[2].parse::<char>().expect("Error Parsing Unit")) {
            Ok(input_unit) => input_unit,
            Err(msg) => {
                println!("{}", msg);
                return;
            }
        };
    let result = convert_temp(temp, &from, &to);
    println!("{} {:?} =  {} {:?}", temp, from, result, to)
}
