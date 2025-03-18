use std::io;
// temp project

fn main() {

    
    loop{
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
            
            get_input_convert("fahrenheit", "celcius");


        } else if choice_user == "C" {

            get_input_convert("celcius", "fahrenheit");

        }
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

fn get_input_convert(from_unit: &str, to_unit: &str)  {
    
    let num_units: i32  = loop {
        let mut user_input = String::new();

        println!("Enter number to convert from {from_unit} to {to_unit}");
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        match user_input.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_e) => {
                eprintln!("Invalid number");
                
            },
        };
    };


    if from_unit == "fahrenheit" {

        let results = fahrenheit_celcius(num_units);
        println!("{num_units}F is {results}C");

    } else if from_unit == "celcius" {

        let results = celcius_fahrenheit(num_units);
        println!("{num_units}C is {results}F");

    } else {

        println!("Invalid choice Try again");

    }
}


