use std::io::Write;
use std::io::Read;

fn main (){
     
    //making the vectors of the data  
    let  commissioner = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let  ministy = vec!["Internal Affairs","Justice", "Defence", "Power & Steel", "Petroleum"];
    let  zone = vec!["South West", "North East", "South South", "South West","South East"];
    
    //making the files
    let mut file1 = std::fs::File::create("commissioners.csv").expect("Failed to create");
    let mut file2 = std::fs::File::create("ministry.csv").expect("Failed to create");
    let mut file3 = std::fs::File::create("zone.csv").expect("Failed to create");

    //getting the files to individual texts that can be put into the file
    for i in 0..commissioner.len(){
        let commissioner_indidvidual = format!("{}\n",commissioner[i]);
        let ministy_individual = format!("{}\n",ministy[i]);
        let zone_individual = format!("{}\n",zone[i]);


        //writing the text to the files 
        file1.write_all(commissioner_indidvidual.as_bytes()).expect("Failed to write to file");
        file2.write_all(ministy_individual.as_bytes()).expect("Failed to write to file");
        file3.write_all(zone_individual.as_bytes()).expect("Failed to write to file");
    }

    //making the Convicted Ministers file 
    let mut file4 = std::fs::File::create("Convicte_Ministers.csv").expect("Failed to create file");
    file4.write_all("Commissioner,Ministry,Zone".as_bytes()).expect("Failed to write");

    //reading the 3 seperated files then storing them into a variable
    
    //commisioner
    let mut file1 = std::fs::File::open("commissioners.csv").expect("Failed to open");
    let mut cf = String::new();
    file1.read_to_string(&mut cf).expect("Faile to read");
    println!("{}",cf );

    //ministry 
    let mut file2 = std::fs::File::open("ministry.csv").expect("Failed to open");
    let mut mf = String::new();
    file2.read_to_string(&mut mf).expect("Faile to read");
    println!("{}",mf );

    //zone
    let mut file3 = std::fs::File::open("zone.csv").expect("Failed to open");
    let mut zf = String::new();
    file3.read_to_string(&mut zf).expect("Faile to read");
    println!("{}",zf );


    //so now i have 3 files that i have read to then get those text that i stored in cf,mf,zf 
    //i have to use .lines() and .collect() which i havent been taught yet so i went to AI for information for this part 
    //.lines() basically turns the one big text into sperated lines 
    //.collect() gathers those lines made into a vector which will thus be stored into the Covicted_Ministers file
    //I noticed this once i reached this certain part of the code and consulted AI if not the only thing i could do is to leave the dataset as files 
    //then put them into the file "Convicted_Minsters"as seen below at the very end of the code
    //Then as stated in project to i used format macro which was also taught to me by AI

    //collecting the values 
    let cl: Vec<&str> = cf.lines().collect();
    let ml: Vec<&str> = mf.lines().collect();
    let zl: Vec<&str> = zf.lines().collect();

    //Writing the values into the Convicted_Ministers file 
    for index in 0..cl.len(){
        let convicted = format!("{},{},{}\n",cl[index],ml[index],zl[index]);
        file4.write_all(convicted.as_bytes()).expect("Failed to write ");
    }
}
   /* let mut file = std::fs::File::create("Convicted_Ministers.csv").expect("Failed to create the file");
    file.writ_all("Commissioner,Minister,Zone".as_bytes()).expect("Failed to write to the file")

    for index in 0..commissioner.len(){
        let  individual = format!("{},{},{}\n",commissioner[index],ministy[index],zone[index]); //as stated in project2 i got this format macro from AI and from that knowledge used it here also
        file.write_all(individual.as_bytes()).expect("Failed to write to the file");


    }

    let mut file = std::fs::File::open("Convicted_Ministers.csv").expect("Failed to open file");
    let mut commissioners = String::new();
    file.read_to_string(&mut commissioners).expect("Failed to read");
    println!("{}",commissioners );*/