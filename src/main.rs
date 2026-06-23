use std::io;

#[derive(Debug)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

const KELVIN_OFFSET: f64 = 273.15;

fn to_celsius(temp: f64, from_unit: &TemperatureUnit) -> f64 {
    match from_unit {
        TemperatureUnit::Fahrenheit => (temp - 32.0) * 5.0 / 9.0,
        TemperatureUnit::Kelvin => temp - KELVIN_OFFSET,
        _ => temp,
    }
}

fn from_celsius(temp: f64, to_unit: &TemperatureUnit) -> f64 {
    match to_unit {
        TemperatureUnit::Fahrenheit => temp * 9.0 / 5.0 + 32.0,
        TemperatureUnit::Kelvin => temp + KELVIN_OFFSET,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let parts: Vec<&str> = input.split_whitespace().collect();
    let [temp, from, to] = parts.as_slice() else {
        return Err("Wrong number of inputs".into());
    };
    let from = convert_to_unit(from.parse::<char>()?)?;
    let to: TemperatureUnit = convert_to_unit(to.parse::<char>()?)?;

    let result = convert_temp(temp.parse::<f64>()?, &from, &to);
    println!("{} {:?} =  {} {:?}", temp, from, result, to);
    Ok(())
}
