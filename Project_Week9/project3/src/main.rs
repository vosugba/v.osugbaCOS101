use std::io::Write;
use std::io::Read;

fn main (){

    let  commissioner = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let  ministy = vec!["Internal Affairs","Justice", "Defence", "Power & Steel", "Petroleum"];
    let  zone = vec!["South West", "North East", "South South", "South West","South East"];

    let mut file = std::fs::File::create("Convicted_Ministers.csv").expect("Failed to create the file");
    file.writ_all("Commissioner,Minister,Zone".as_bytes()).expect("Failed to write to the file")

    for index in 0..commissioner.len(){
        let  individual = format!("{},{},{}\n",commissioner[index],ministy[index],zone[index]); //as stated in project2 i got this format macro from AI and from that knowledge used it here also
        file.write_all(individual.as_bytes()).expect("Failed to write to the file");


    }

    let mut file = std::fs::File::open("Convicted_Ministers.csv").expect("Failed to open file");
    let mut commissioners = String::new();
    file.read_to_string(&mut commissioners).expect("Failed to read");
    println!("{}",commissioners );


}
