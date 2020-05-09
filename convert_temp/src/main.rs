use std::io;

fn main() {
    let final_choice;

    loop {
        println!(
            "Convert from:
        1. Fahrenheit to Celsius
        2. Celsius to Fahrenheit"
        );

        println!("Enter your choice.");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u8 = match choice.trim().parse() {
            Ok(choice) => {
                if choice == 1 || choice == 2 {
                    choice
                } else {
                    println!("Invalid choice! Try again.");
                    continue;
                }
            }
            Err(_) => {
                println!("Invalid choice! Try again.");
                continue;
            }
        };

        final_choice = choice;
        break;
    }

    let final_temp;

    loop {
        println!("Enter the temperature.");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(temp) => temp,
            Err(_) => continue,
        };

        final_temp = temp;
        break;
    }

    if final_choice == 1 {
        println!(
            "{} degrees F is {} degrees C.",
            final_temp,
            f_to_c(final_temp)
        );
    } else {
        println!(
            "{} degrees C is {} degrees F.",
            final_temp,
            c_to_f(final_temp)
        );
    }
}

fn f_to_c(t: f64) -> f64 {
    (t - 32.0) * 5.0 / 9.0
}

fn c_to_f(t: f64) -> f64 {
    (t * 9.0 / 5.0) + 32.0
}
