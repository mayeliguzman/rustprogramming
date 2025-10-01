//declaring const for the freezing point 
const FREEZING_POINT_F: f64 = 32.0;

// implementing the two functions
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn main() {
    //declare mutable variable
    let mut temp_f: i32 = 32;
    temp_f += 1;

    println!("Starting temperature: {} degrees F", temp_f);
    let temp_c = fahrenheit_to_celsius(temp_f as f64);
    println!("{} degrees F is {:.2} degrees C", temp_f, temp_c);

    // loop to convert and print next 5 integer temps
    for i in 1..=5 {
        let next_f = temp_f + i;
        let next_c = fahrenheit_to_celsius(next_f as f64);
        println!("{} degrees F is {:.2} degrees C", next_f, next_c);
    }
}
