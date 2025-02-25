use std::io;


fn main() {
    println!("Exercises");

    pg58ex1();
}

fn pg58ex1(){
    let temperature: i32; //u8; //possibly use i32 instead if there's problems
    let mut input = int::new();
    println! ("");
    println!("Convert to celsius(1) or fahrenheit(2)?:");
    println! ("");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    if input == 1{
        println!("Please enter the temperature:");    
        let mut input: i32;
        io::stdin()
        .read_line (&mut input)
        .expect ("Failed to read input");
        let newtemp: String=temperature+32;
        println!("{ }", &str(newtemp));
    }

    if input == 2{
        println!("Please enter the temperature:");
        let mut input2: i32;
        io::stdin()
        .read_line (&mut input2)
        .expect("Failed to read input");
        let newtemp: String=temperature-32;
        println!("{ }", &str(newtemp));
    }
}


fn pg58ex2(){

}

fn pg58ex3(){

}



