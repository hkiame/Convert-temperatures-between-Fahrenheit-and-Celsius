use std::io;

fn main() {
    println!("Convert temperatures between Fahrenheit and Celsius.");
    println!("Please specify the value you are entering as either Fahrenheit or Celsius");
    let mut inp = String::new();
    let mut value = String::new();

    io::stdin().read_line(&mut inp).expect("Failed to read input");
    println!("Please provide the value");
    io::stdin().read_line(&mut value).expect("Failed to read input");
    let value: i32 = match value.trim().parse(){
        Ok(num) => num,
        Err(_) => 0
    };
    let inp = inp.trim();
    if inp == "Fahrenheit" {
        let result: i32 = (value - 32) * 5/9;
        println!("The result is {}°F", result);
        
    } else if inp == "Celsius" {

        let result: i32 = (value * 9/5) + 32;
        println!("The result is {}°C", result);
    }

}
