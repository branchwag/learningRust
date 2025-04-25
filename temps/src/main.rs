use std::io;

fn main() {
    let mut temp_input = String::new();
    let mut unit_input = String::new();

    println!("Enter the temperature:");
    io::stdin()
        .read_line(&mut temp_input)
        .expect("Failed to read temperature");
    let temp: f64 = match temp_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    //selector for F or C
    println!("Enter the unit (F for Fahrenheight, C for Celsius):");
    io::stdin()
        .read_line(&mut unit_input)
        .expect("Failed to read unit");
    let unit = unit_input.trim().to_uppercase();

    let converted = if unit == "F" {
        (temp - 32.0) * 5.0 / 9.0
    } else if unit == "C" {
        (temp * 9.0 / 5.0) + 32.0
    } else {
        println!("Invalid unit. Please enter F or C.");
        return;
    };

    println!("Converted temperature: {:.2}", converted);
}
