fn main() {
    
    // Using Vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the size of the vector
    println!("\n The length of Vec::new is: {}",v.len() );

    //using macro
    let v = vec!["grace","Effiong","Basil","Kaareem","Susan"];

    //printing the size of the vector
    println!("\n The length of vec macro is: {}",v.len() );
}
