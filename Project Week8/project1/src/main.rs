/* we make the table just put it as it is for display only nothing to do with the code then a variable table with a vector with tuples
i would have used just tuples but the the question asked fot the use of vectors i used one then using the for loop in the table i connect 
the code inputted in the read_code function to the title level and category then print it out.
Now if the person types nonsense what do we do idk we will figure that out later 
(please not if this last statment is still there i didn't figure it out )*/


fn read_Code () -> String{
    println!("Welcome to the APS level checker from the table input the code that fit you best");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read this input abeg input another");
    let read_code = input.trim().to_uppercase().to_string();
    read_code
}

fn main (){
    println!("");
    println!("=============================================================================================================================================");
    println!("|CATEGORY      TITLE1          TITLE2                 TITLE3             TITLE4                    TITLE5                    TITLE6          |");       
    println!("|OFFICE ADMIN  Intern(OI)      Admistrator(OA)        Senior Admin(OS)   Office Manager(OO)        Director (OD)             CEO(OC)         |");
    println!("|ACADEMIC      --------        Reaserch Assistant(AR) PhD Candidate(APh) Post-Doc Reasearcher(APo) Senior Lectuer(AS)        Dean(AD)        |");
    println!("|LAWYER        Paralegal(LP1)  Junior Associate(LJ)   Associate(LA)      Senior Associate1-2(LA1)  Senior Associate3-4(LA2)  Partner(LP2)    |");
    println!("|TEACHER       Placement(TP1)  Classroom Teacher(TC)  Senior Teacher(TS) Leading Teacher(TL)       Deputy Principal(TD)      Principal(TP2)  |");
    println!("|LEVEL         APS 1-2         APS 3-5                APS 5-8            EL1 8-10                  EL2 10-13                 SES             |");
    println!("=============================================================================================================================================");
    println!("");
    

    let mut table : Vec<(&str, &str, &str)> = Vec::new();
    table.push(("OI","Intern","APS 1-2"));
    table.push(("OA","Admistrator","APS 3-5"));
    table.push(("OS","Senior Administrator","APS 5-8"));
    table.push(("OO","Office Manger","EL1 8-10"));
    table.push(("OD","Director","El2 10-13"));
    table.push(("OC","CEO","SES"));
    table.push(("AR", "Reaserch Assistant", "APS 3-5"));
    table.push(("APh", "PhD Candidate", "APS 5-8" ));
    table.push(("APo", "Post-Doc Reasearcher", "EL1 8-10"));
    table.push(("APS", "Senior Lectuer", "EL2 10-13"));
    table.push(("AD", "Dean", "SES"));
    table.push(("LP1", "Paralegal", "APS 1-2"));
    table.push(("LJ", "Junior Associate", "APS 3-5"));
    table.push(("LA", "Associate", "APS 5-8"));
    table.push(("LA1", "Senior Associate1-2", "EL1 8-10"));
    table.push(("LA2", "Senior Associate3-4", "EL2 10-13"));
    table.push(("LP2", "Partner", "SES"));
    table.push(("TP1", "Placement", "APS 1-2"));
    table.push(("TC", "Classroom Teacher", "APS 3-5"));
    table.push(("TS", "Senior Teacher", "APS 5-8"));
    table.push(("TL", "Leading Teacher", "EL1 8-10"));
    table.push(("TD", "Deputy Principal", "EL2 10-13"));
    table.push(("TP2", "Principal", "SES"));

    let code =read_Code();
    let code = code.as_str().to_uppercase();

    let mut found = false;

    for (c,title,level) in table.iter(){
    if *c == code{
        println!("Your Job Title is {} and level is {}",title,level );
        found = true;
    }
    
}
      if found == false{
        println!("Not found");
    }
}