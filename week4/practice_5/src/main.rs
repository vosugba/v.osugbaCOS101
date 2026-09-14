//Rust program to read the height of a person and then print if the person is tall, dwarf, or average height person 

use std::io;

 fn main() {
    let mut input = String::new();


    println!("Enter Your Height (in centimeters):");
    io::stdin()
    .read_line(&mut input)
    .expect("Not a vaild string ");

    let height:f32 = input
    .trim()
    .parse()
    .expect("Not a vaild number");

    if height >=150.0 && height <= 170.00{
        println!("You are of average height");
    } 
    else if height > 170.00 && height <= 195.00{
        println!("Your are tall");

    }
    else if height < 150.00 && height >100.00{
        println!("Nahh G u are short fahhhh");


    }
    else {
        println!("Abnormal height ");
    }
}