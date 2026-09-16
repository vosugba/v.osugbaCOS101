// Quadratic formula calculator
/* To input the 3 values needed "a", "b", and "c"
then show if the result will give "2 roots", "one root" or "no root"
then solve the equation and give the result */

use std::io;

fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("The quadratic equation = ax^2 + bx + c =0");

    


    println!("Input you a");
    io::stdin()
    .read_line(&mut input1)
    .expect("Failed to read input");

    let a:f64= input1
    .trim()
    .parse()
    .expect("Failed to read value");

    println!("Input you b");
    io::stdin()
    .read_line(&mut input2)
    .expect("Failed to read input");

    let b:f64= input2
    .trim()
    .parse()
    .expect("Failed to read value");

    println!("Input you c");
    io::stdin()
    .read_line(&mut input3)
    .expect("Failed to read input");

    let c:f64= input3
    .trim()
    .parse()
    .expect("Failed to read value");

    
    
    let d:f64 = b*b - 4.0*a*c;


    println!("Lets check how many roots there will be using the determinant (d)= b*b - 4.0*a*c ");

    

    if d > 0.0{
        let roots:f64 = (-b + d.sqrt())/2.0*a ;
        let root2:f64 =(-b - d.sqrt())/2.0*a ;
        println!("Two distinct roots");

        println!("So the roots of the equations are {} and {}", roots, root2);
    }
    else if d == 0.0{
        let roots:f64 = -b /2.0*a ;
 
        println!("One real root");
        println!("So the roots of the equations are {}", roots)
    }
    else if d < 0.0 {
        println!("No roots ");


    }
  
    




}