use std::io;

fn pg58ex1() {
    let mut input = String::new();

    println!("\nConvert to Celsius (1) or Fahrenheit (2)?:");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let choice: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter 1 or 2.");
            return;
        }
    };

    if choice == 1 {
        let mut temp_input = String::new();
        println!("Please enter the temperature in Fahrenheit:");

        io::stdin()
            .read_line(&mut temp_input)
            .expect("Failed to read input");

        let temperature: i32 = match temp_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature input.");
                return;
            }
        };

        let newtemp = (temperature - 32) * 5 / 9; // Fahrenheit to Celsius conversion
        println!("Temperature in Celsius: {}", newtemp);
    } else if choice == 2 {
        let mut temp_input = String::new();
        println!("Please enter the temperature in Celsius:");

        io::stdin()
            .read_line(&mut temp_input)
            .expect("Failed to read input");

        let temperature: i32 = match temp_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature input.");
                return;
            }
        };

        let newtemp = (temperature * 9 / 5) + 32; // Celsius to Fahrenheit conversion
        println!("Temperature in Fahrenheit: {}", newtemp);
    } else {
        println!("Invalid choice. Please enter 1 or 2.");
    }
}


fn pg58ex2() {
    


}    

fn main() {
//    pg58ex1();
    pg58ex2();

}

