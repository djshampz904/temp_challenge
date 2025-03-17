use std::io;
// temp project

fn main() {

    let mut user_input = String::new();
    let mut user_choice = String::new();
    
    println!("Enter the units you are converting from: example (F) to convert from Fahrenheit to celcius (F/C)");
    io::stdin().read_line(&mut user_choice).expect("Failed to read line");
 
    let choice_user = user_choice.trim().to_uppercase();
    match choice_user.as_str() {
        "F" => println!("User picked fahrenheit"),
        "C" => println!("User picked celcius"),
        _   => println!("Invalid choice: options F/C"),
    };

    if choice_user == "F" {
        println!("Enter number to convert from fahrenheit to celcius");
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let mut num_units: i32 = user_input.trim().parse().expect("Failed to parse value");
        let mut results = fahrenheit_celcius(num_units);
        println!("{num_units}F is {results}C");
    } else if choice_user == "C" {
        println!("Enter number to convert from Celcius to fahrenheit");
        io::stdin().read_line(&mut user_input).expect("Failed to parse value");

        let mut num_units: i32 = user_input.trim().parse().expect("Failed to parse value");
        let mut results = celcius_fahrenheit(num_units);
        println!("{num_units}C is {results}F");
    }


}


fn fahrenheit_celcius(value: i32) -> i32 {
    let celcius: i32 = (value - 32) * 5/9;
    return celcius;
}

fn celcius_fahrenheit(value: i32) -> i32 {
    let fahrenheit: i32 = (value * 9/5) + 32;
    return fahrenheit;
}
