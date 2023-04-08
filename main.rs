use std::io;
use std::io::Write;

fn main() {
    let mut rates: f64;
    let mut final_price: f64;
    let mut weight: i64 = 0;
    let mut miles: i64 = 0;

    loop {
        println!("Whats the weight in kg: ");
        io::stdout().flush().unwrap();

        let mut weight_given = String::new();
        io::stdin()
            .read_line(&mut weight_given)
            .expect("Failed to read input");

        let weight = match weight_given.trim().parse::<i32>() {
            Ok(num) if num > 0 && num <= 20 => {
                break;
            }
            _ => {
                println!("Invalid input, give me an integer greater than 0 but less than 20");
                continue;
            }
        };
    }

    loop {
        println!("How many miles is it traveling: ");
        io::stdout().flush().unwrap();
        let mut miles_given = String::new();
        io::stdin()
            .read_line(&mut miles_given)
            .expect("Failed to read input");

        let miles = match miles_given.trim().parse::<i32>() {
            Ok(num) if num > 10 && num <= 3_000 => {
                break;
            }
            _ => {
                println!("Invalid input, give me an integer greater than 10 but less than or equal to 3,000");
                continue;
            }
        };
    }


    match weight {
        0..=2 => {
            rates = 1.1;
        }

        3..=6 => {
            rates = 2.2;
        }

        7..=10 => {
            rates = 3.7;
        }

        11..=20 => {
            rates = 4.8;
        }
        _ => {
            rates = 0.0;
        }
    }

    if miles >= 500 {
        final_price = (miles as f64 / 500.0) * rates;
        println!("Final price is {final_price}");
    } else {
        println!("final price is 0");
    }
}

