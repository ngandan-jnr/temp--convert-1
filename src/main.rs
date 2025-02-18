

use std::process;
use std::io::{self, Write};

fn main() {
    print!("Enter the temperature and scale (e.g., 100 C or 32 F): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        eprintln!("Usage: <temperature> <scale (C/F/K)>");
        process::exit(1);
    }

    let temp_value = parse_temperature(parts[0]);
    let scale = parse_scale(parts[1]);

    if let Some((temp_c, temp_f, temp_k)) = convert_temperature(temp_value, scale) {
        println!("Temp in C: {:.2}", temp_c);
        println!("Temp in F: {:.2}", temp_f);
        println!("Temp in K: {:.2}", temp_k);
    } else {
        process::exit(1);
    }
}

fn parse_temperature(temp_str: &str) -> i32 {
    temp_str.parse().unwrap_or_else(|_| {
        eprintln!("Invalid temperature value. Please provide an integer.");
        process::exit(1);
    })
}

fn parse_scale(scale_str: &str) -> char {
    let scale = scale_str.to_uppercase();
    if !['C', 'F', 'K'].contains(&scale.chars().next().unwrap()) {
        eprintln!("Invalid scale. Use 'C', 'F', or 'K'.");
        process::exit(1);
    }
    scale.chars().next().unwrap()
}

fn convert_temperature(temp: i32, scale: char) -> Option<(f64, f64, f64)> {
    match scale {
        'C' => Some((temp as f64, (temp as f64 * 9.0 / 5.0) + 32.0, temp as f64 + 273.15)),
        'F' => Some(((temp as f64 - 32.0) * 5.0 / 9.0, temp as f64, (temp as f64 - 32.0) * 5.0 / 9.0 + 273.15)),
        'K' => Some((temp as f64 - 273.15, (temp as f64 - 273.15) * 9.0 / 5.0 + 32.0, temp as f64)),
        _ => None,
    }
}
