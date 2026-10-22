use std::io::Read;
use std::io::Write;


fn name (question1: &str) -> String {
	println!("{}",question1 );
	let mut input = String::new();

	std::io::stdin().read_line(&mut input).expect("Failed to read input");
	input.trim().to_uppercase()
	
}
fn matric (question2: &str) -> String {
	println!("{}",question2 );
	let mut input1 = String::new();

	std::io::stdin().read_line(&mut input1).expect("Failed to read input");
	input1.trim().to_uppercase()

}
fn department (question3: &str) -> String {
	println!("{}",question3 );
	let mut input2 = String::new();

	std::io::stdin().read_line(&mut input2).expect("Failed to read input");
	input2.trim().to_uppercase()
	
}
fn level (question4: &str) -> i32 {
	println!("{}",question4 );
	let mut input3 = String::new();

	std::io::stdin().read_line(&mut input3).expect("Failed to read input");
	input3.trim().parse().expect("Failed to read input")
	
}
fn read () {
	let mut file = std::fs::File::open("project2_files.csv").expect("Failed to open");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Failed to read");
	println!("{}",contents );
}




fn main () {
	
    let mut details : Vec <(String,String,String,i32)> = Vec::new();
	
    loop{
	println!("WELCOME STUDENT TO THE SMIS STUDENT DETAILS RECORDS");
	println!("Please input your fullname, matric number, department and level");

	let name = name("Input your fullname");

	let matric = matric("Please enter your matric number");

	let department = department("Please enter your department");

	let level = level("Input your level in your department");

	
	details.push((name, matric, department, level));

	print!("Do you want to input another students detail (y/n)\n");

	let mut choice = String::new();
	std::io::stdin().read_line(&mut choice).expect("Failed to get your choice");
	let choice = choice.trim().to_uppercase();

	if choice == "N"  {
		println!("Thankyou for your input");

		let mut file = std::fs::File::create("project2_files.csv").expect("Failed to create");
		file.write_all("Students Name,Matric number,Department,Level".as_bytes()).expect("Failed to write");

		for (n,m,d,l) in details.iter(){
			let individual = format!("{},{},{},{}\n",n,m,d,l);
			file.write_all(individual.as_bytes()).expect("Failed to write"); 
		}
		read();
		break 
	}else if choice == "Y" {
		println!("Okay input again");
	}else {
		println!("Not a valid input ");



/*the for loop was AI it kept showing that it couldnt input the vector as it is into the file and i didnt understand
the only new thing to me was the format macro and i now understand what it does it basically takes several values and joins them into one string
it stores them into a text its kind of like println! but doesnt print it out but sores it into a variable */
	}

}
	

}


	