use clap::{Arg, Command};
use std::process;

fn 
    let matches = Command::new("Temperature Converter")
        .version("1.0")
        .author("Your Name")
        .about("Converts temperatures between Celsius, Fahrenheit, and Kelvin.")
        .arg(Arg::new("temperature")
            .help("Temperature value followed by C, F, or K (e.g., 100C, 212F, 373K)")
            .required(true)
            .index(1))
        .arg(Arg::new("celsius")
            .short('c')
            .help("Display temperature in Celsius"))
        .arg(Arg::new("fahrenheit")
            .short('f')
            .help("Display temperature in Fahrenheit"))
        .arg(Arg::new("kelvin")
            .short('k')
            .help("Display temperature in Kelvin"))
        .arg(Arg::new("all")
            .long("all")
            .help("Display temperature in all units"))
        .get_matches();
    
    let input = matches.get_one::<String>("temperature").unwrap();
    
    if let Some((temp_c, temp_f, temp_k)) = convert_temperature(input.trim()) {
        if matches.contains_id("all") {
            print_converted_temps(temp_c, temp_f, temp_k);
        } else {
            if matches.contains_id("celsius") {
                println!("Temp in C: {:.2}", temp_c);
            }
            if matches.contains_id("fahrenheit") {
                println!("Temp in F: {:.2}", temp_f);
            }
            if matches.contains_id("kelvin") {
                println!("Temp in K: {:.2}", temp_k);
            }
        }
    } else {
        process::exit(1);
    }


fn convert_temperature(input: &str) -> Option<(f64, f64, f64)> {
    if input.len() < 2 {
        eprintln!("Invalid input. Please enter a number followed by 'C', 'F', or 'K'.");
        return None;
    }

    let (num_part, scale) = input.split_at(input.len() - 1);
    let temp: f64 = match num_part.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number format.");
            return None;
        }
    };

    let scale = scale.to_uppercase();
    let (temp_c, temp_f, temp_k) = match scale.as_str() {
        "C" => (temp, (temp * 9.0 / 5.0) + 32.0, temp + 273.15),
        "F" => ((temp - 32.0) * 5.0 / 9.0, temp, (temp - 32.0) * 5.0 / 9.0 + 273.15),
        "K" => (temp - 273.15, (temp - 273.15) * 9.0 / 5.0 + 32.0, temp),
        _ => {
            eprintln!("Invalid scale. Use 'C', 'F', or 'K'.");
            return None;
        }
    };

    Some((temp_c, temp_f, temp_k))
}

fn print_converted_temps(temp_c: f64, temp_f: f64, temp_k: f64) {
    println!("Temp in C: {:.2}", temp_c);
    println!("Temp in F: {:.2}", temp_f);
    println!("Temp in K: {:.2}", temp_k);
}
