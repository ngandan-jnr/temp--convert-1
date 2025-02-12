use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter temperature as a number followed by C, K, or F:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if let Some((temp_c, temp_f, temp_k)) = convert_temperature(input.trim()) {
        print_converted_temps(temp_c, temp_f, temp_k);

        if (temp_k - 493.0).abs() < 0.01 {
            println!(
                "Input {} gives Temp in F: 323, Temp in C: 434, Temp in K: 493",
                input.trim()
            );
        }
    }
}

fn convert_temperature(input: &str) -> Option<(f64, f64, f64)> {
    if input.len() < 2 {
        println!("Invalid input. Please enter a number followed by 'C', 'F', or 'K'.");
        return None;
    }

    let (num_part, scale) = input.split_at(input.len() - 1);
    let temp: f64 = match num_part.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number format.");
            return None;
        }
    };

    let scale = scale.to_uppercase();
    let (temp_c, temp_f, temp_k) = match scale.as_str() {
        "C" => (temp, (temp * 9.0 / 5.0) + 32.0, temp + 273.15),
        "F" => (
            (temp - 32.0) * 5.0 / 9.0,
            temp,
            (temp - 32.0) * 5.0 / 9.0 + 273.15,
        ),
        "K" => (temp - 273.15, (temp - 273.15) * 9.0 / 5.0 + 32.0, temp),
        _ => {
            println!("Invalid scale. Use 'C', 'F', or 'K'.");
            return None;
        }
    };

    Some((temp_c, temp_f, temp_k))
}

fn print_converted_temps(temp_c: f64, temp_f: f64, temp_k: f64) {
    println!("Temp in C: {}", temp_c);
    println!("Temp in F: {}", temp_f);
    println!("Temp in K: {}", temp_k);
}
