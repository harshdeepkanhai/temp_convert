use std::io;

#[derive(Debug)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

fn to_celsius(temp: f64, from_unit: &TemperatureUnit) -> f64 {
    match from_unit {
        TemperatureUnit::Fahrenheit => (temp - 32.0) * 5.0 / 9.0,
        TemperatureUnit::Kelvin => temp - 273.15,
        _ => temp,
    }
}

fn from_celsius(temp: f64, to_unit: &TemperatureUnit) -> f64 {
    match to_unit {
        TemperatureUnit::Fahrenheit => temp * 9.0 / 5.0 + 32.0,
        TemperatureUnit::Kelvin => temp + 273.15,
        _ => temp,
    }
}

fn convert_temp(temp: f64, from_unit: &TemperatureUnit, to_unit: &TemperatureUnit) -> f64 {
    from_celsius(to_celsius(temp, from_unit), to_unit)
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
