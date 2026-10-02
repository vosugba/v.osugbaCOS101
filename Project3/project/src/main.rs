use std::io;
fn main (){
    let menu =[
    ("P","Pounded Yam / Edinkaiko Soup",3_200.00),
    ("F","Fried Rice & Chicken",3_000.00),
    ("A","Amala & Ewedu Soup",2_500.00),
    ("E","Eba & Egusi Soup",2_000.00),
    ("W","White Rice & Stew",2_500.00),
           
    ];
    let mut name = String::new();
        println!("Please enter your full name");
       io::stdin().read_line(&mut name).expect("Failed to read name");
        let name = name.trim();

    println!("WELCOME {} TO ICHIRAKU CUSINE HERE IS OUR MENU",name );
    println!("Here is out menu:{:?}",menu );

    let mut orders:Vec<(&str,f64)> = Vec::new();
    let mut total:f64 = 0.00;

    loop{
        println!("INPUT YOUR ORDER CODE FROM THE MENU OR TYPE DONE ONCE FINSHED  ");
        let mut meal_choice = String::new();
        io::stdin().read_line(&mut meal_choice).expect("Failed to read meal choice code");
        let meal_choice = meal_choice.trim().to_uppercase();

        if meal_choice == "DONE"{
            println!("Thankyou for ordering ");
            break;
        }
    

    let found = menu.iter().find(|(code,_,_)| *code == meal_choice);

    match found {
        Some((_,meal_name, price))=> {
            println!("Enter the quantity for your order");
            let mut qty = String::new();
            io::stdin().read_line(&mut qty).expect("Failed to read the quantity");
            let qty:f64 = qty.trim().parse().expect("Failed to read quantity");

            let mut order_total = price*qty;

            if order_total > 10_000.00{
                order_total *= 0.95;
                println!("Discounted total for the order is : {} ", order_total);


            }else{
                println!("Order total is: {}",order_total);
            }

           orders.push((meal_name, order_total));
           total += order_total


        }
         None=> {
                println!("Not a valid code, try again")
            }
    }
}
    println!("\n Order summary:");
    for (meal , cost) in &orders{
        println!("{} - {:.2}",meal,cost );
    }
    println!("Total: {:.2}",total );

   
}