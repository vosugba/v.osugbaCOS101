fn main() {
    let fullname = "Chibudum John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";



    let mut school = "School of Science".to_string();

    //push string
    school.push_str("  and Technology");

    println!("My naem is: {}",fullname );
    //check length 
    println!("The length of my fullname is: {}", fullname.len());
    println!("{}",school );
    println!("{:?}",uni ); 
}
