use core::fmt;
use std::{fmt::Result, io};

fn main() {
    let mut input = String::new();

    'celcius: loop {
        println!("Please input a temperature in Celcius");
        match io::stdin().read_line(&mut input) {
            Err(_) => {
                println!("Failed to read line");
            }
            Ok(_) => {}
        }

        match input.trim().parse::<f64>() {
            Err(_) => {
                println!("Not a valid integer");
            }
            Ok(result_value) => {
                let input_celcius = TemperatureScale::Celcius(result_value);
                let input_fahrenheit: TemperatureScale = convert_to_fahrenheit(&input_celcius);
                let input_kelvin: TemperatureScale = convert_to_kelvin(&input_celcius);

                println!("{}", input_celcius);
                println!("{}", input_fahrenheit);
                println!("{}", input_kelvin);
                break 'celcius;
            }
        }
    }
}
pub enum TemperatureScale {
    Celcius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

impl fmt::Display for TemperatureScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemperatureScale::Celcius(temp) => {
                write!(f, "Temperature is {temp} in degrees Celcius.")
            }
            TemperatureScale::Fahrenheit(temp) => {
                write!(f, "Temperature is {temp} in degrees Fahrenheit.")
            }
            TemperatureScale::Kelvin(temp) => {
                write!(f, "Temperature is {temp} in degrees Kelvin.")
            }
        }
    }
}
pub fn convert_to_celcius(temperature: &TemperatureScale) -> TemperatureScale {
    match temperature {
        TemperatureScale::Celcius(temp) => {
            return TemperatureScale::Celcius(*temp);
        }
        TemperatureScale::Fahrenheit(temp) => {
            return TemperatureScale::Celcius((*temp - 32.0) / 1.8);
        }
        TemperatureScale::Kelvin(temp) => {
            return TemperatureScale::Celcius(temp + 273.15);
        }
    }
}
pub fn convert_to_fahrenheit(temperature: &TemperatureScale) -> TemperatureScale {
    match temperature {
        TemperatureScale::Celcius(temp) => {
            return TemperatureScale::Fahrenheit(*temp * 1.8 + 32.0);
        }
        TemperatureScale::Fahrenheit(temp) => {
            return TemperatureScale::Fahrenheit(*temp);
        }
        TemperatureScale::Kelvin(temp) => {
            return TemperatureScale::Fahrenheit((*temp + 273.15) * 1.8 + 32.0);
        }
    }
}
pub fn convert_to_kelvin(temperature: &TemperatureScale) -> TemperatureScale {
    match temperature {
        TemperatureScale::Celcius(temp) => {
            return TemperatureScale::Kelvin(*temp - 273.15);
        }
        TemperatureScale::Fahrenheit(temp) => {
            return TemperatureScale::Kelvin(*temp * 1.8 + 32.0 + 273.15);
        }
        TemperatureScale::Kelvin(temp) => {
            return TemperatureScale::Kelvin(*temp);
        }
    }
}
