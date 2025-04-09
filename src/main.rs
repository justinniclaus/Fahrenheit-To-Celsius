use std::io;

fn main() {
    
    let mut input = String::new(); 
    let mut value = String::new();

    println!("Farenheit or Celsius?");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let input: &str = input
        .trim();

    println!("Enter temperature value: ");

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read value.");

    let value: f32 = value
        .trim()
        .parse()
        .expect("Failed to convert from string to floating point.");

    if input == "f" || input == "F" || input == "farenheit" || input == "Farenheit"
    {
       let c = (value - 32.0) / 1.8;
        
        println!("Farenheit {:.1} to Celsius {:.1}.", value, c);
    }
    else if input == "c" || input == "C" || input == "celsius" || input == "Celsius"
    {
        let f = ( value * 1.8) + 32.0;

        println!("Celsius {:.1} to Farenheit {:.1}.", value, f);
    }
    else
    {
        println!("Input is invalid.");
    }
}
