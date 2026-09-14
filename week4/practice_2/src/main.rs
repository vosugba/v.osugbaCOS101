// Rust program to calcualte the area of a triangle given three sides

use std::io;

fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

     println!("Enter first edge of triangle: "); // The string to direct the user to input the first number
    io::stdin() // 
    .read_line(&mut input1) // read the line that the user has placed in input1 '(&mut input1)' points to it
    .expect("Not a valid string");// in case of errors

    let a:f32 = input1 //assigning a variable to whatever was typed in input1
    .trim() // removal of spaces 
    .parse() // type casting from string to f32
    .expect("Not a valid number"); // in case of errors

     println!("Enter second edge of triangle: "); // The string to direct the user to input the second number
    io::stdin() // 
    .read_line(&mut input2) // read the line that the user has placed in input2 '(&mut input2)' points to it
    .expect("Not a valid string");// in case of errors

    let b:f32 = input2 //assigning a variable to whatever was typed in input2
    .trim() // removal of spaces 
    .parse() // type casting from string to f32
    .expect("Not a valid number"); // in case of errors


     println!("Enter third edge of triangle: "); // The string to direct the user to input the third number
    io::stdin() // 
    .read_line(&mut input3) // read the line that the user has placed in input3 '(&mut input3)' points to it
    .expect("Not a valid string");// in case of errors

    let c:f32 = input3//assigning a variable to whatever was typed in input3
    .trim() // removal of spaces 
    .parse() // type casting from string to f32
    .expect("Not a valid number"); // in case of errors


    
    let s:f32 = (a + b + c)/2.0;
    let mut area:f32 = s * (s-a)*(s-b)*(s-c);
    area = area.sqrt();

    println!("The Area of a triangle :{:?}",area);

}