fn name () -> String {
    print!("Welcome prospect employee to the Ernst & Young (EY) Global limited experince checker\n  ");
    println!("please input you Name and experince"); 
    
    //getting name from the user
    println!("Please input your full name");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read");
    let name = input1.trim().to_uppercase();

    return name;
}

fn experience () -> u32 {
    //getting the individuals experience 
    println!("Please input your experience");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let experience:u32 = input2.trim().parse().expect("Failed to read experince");
    return experience;
}

fn main (){
   let mut vector: Vec<(String , u32)> = Vec::new();

   loop{
   
    let n: String = name();
    let e: &u32= &experience(); 
   
    let candidate :(String, u32) = (n ,*e);
    vector.push(candidate);

    println!("Is there another candidates y/n");
    let mut choice= String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read choice");
    let choice = choice.trim().to_lowercase();

    if choice == "n"{
        println!("Thank you for inputing");
         println!("The list of Candidates {:?}\n",vector );
        break;
    }else if choice == "y"{
        println!("Okay continue");
    }else{
        println!("not a correct value put either y or n")
    }

   

}

 let mut top = &vector[0];

for candidate in &vector {
    if candidate.1 > top.1 {
        top = candidate;

        println!("The person with the highest experience is {} with {} yrears of experience",top.0,top.1 );
    }
}
}

