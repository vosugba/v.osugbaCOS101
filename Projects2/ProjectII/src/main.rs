//The Incentive Calculator
//Inputing the age of employees
//checking if the employees are experienced 

use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Please input your age:");
    io::stdin()
    .read_line(&mut input2) //this is a pointer pointing to the input 2 in order for it to be read
    .expect("Failed to read string try again!"); 

    let age:u32 = input2 //this is to assign what ever string that was typed in input2 to the variable "Age"
    .trim() // this is to remove spaces
    .parse()// this is to covert the data type from a string to an unsigned interger as stated earlier by puting "u32"
    .expect("Failed to read age try again"); // this will print just in case of an error

    println!("Are you expericed: Type (y) for Yes or (n) for No");
    io::stdin()
    .read_line(&mut input1)
    .expect("Failed to read input");

    let experience = input1
    .trim().to_lowercase();

    let experienced = experience == "y";
    

    if experienced {
        if age <28 {
        println!("Congratulations your Incentive is 1,300,000 Naira ");
       }
        else if age <=39 {
        println!("Congratulations your Incentive is 1,480,000 Naira");
       }
        else if age >=40 {
         println!("Congratulations your Incentive is 1,560,000 Naira");
       }
        
    }
        else if experience == "n" {
        println!("Congratulations your Incentive is 100,000 Naira");
      } 
       else{
            println!("No Incentive");
       }

}    