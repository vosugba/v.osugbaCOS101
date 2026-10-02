use std::io;
    
    fn main() {
        loop{
        let p:(&str,&str,f64) = ("P","Pounded Yam / Edinkaiko Soup",3_200.00);
        let f:(&str,&str,f64) = ("F","Fried Rice & Chicken",3_000.00);
        let a:(&str,&str,f64) = ("A","Amala & Ewedu Soup",2_500.00);
        let e:(&str,&str,f64) = ("E","Eba & Egusi Soup",2_000.00);
        let w:(&str,&str,f64) = ("W","White Rice & Stew",2_500.00);
        let c:(&str,&str,&str) = ("CODE","MEALS","PRICES");

        let mut name = String::new();
        println!("Please enter your full name");

        io::stdin().read_line(&mut name).expect("Failed to read name");
        let name = name.trim();
        println!("WELCOME {} TO ICHIRAKU CUSINE HERE IS OUR MENU",name);
        println!("==========THE ICHIRAKU CUSINE MENU==========
                  {:?}
                  {:?}
                  {:?}
                  {:?}
                  {:?}
                  {:?}
                  Please {:?} input the code of what you want to order",c,p,f,a,e,w,name );

        println!("Please {:?} input code",name );
        let mut meal_choice = String::new();
        io::stdin().read_line(&mut meal_choice).expect("Failed to read your meal choice");
        let meal_choice = meal_choice.trim().to_uppercase();

        println!("The quantity if your order");
        let mut qty = String::new();
        io::stdin().read_line(&mut qty).expect("Failed to read quantity");
        let qty:f64 = qty.trim().parse().expect("Failed to read quantity");

        if meal_choice == "P"{
            let price = p.2*qty;
            println!("Your total is {:?}",price );
            if price > 10_000.00{
                let new_price = price* 0.05;
                println!("Your discounted price is {:?}",new_price );
            }
        }else if meal_choice == "F"{
            let price = f.2*qty;
            println!("Your total is {:?}",price );
            if price > 10_000.00{
                let new_price = price* 0.05;
                println!("Your discounted price is {:?}",new_price );
            }
        }else if meal_choice == "A"{
            let price = a.2*qty;
            println!("Your total is {:?}",price );
            if price > 10_000.00{
                let new_price = price* 0.05;
                println!("Your discounted price is {:?}",new_price );
            }
        }else if meal_choice == "E"{
            let price = e.2*qty;
            println!("Your total is {:?}",price );
            if price > 10_000.00{
                let new_price = price* 0.05;
                println!("Your discounted price is {:?}",new_price );
            }
        }else if meal_choice == "W"{
            let price = w.2*qty;
            println!("Your total is {:?}",price );
            if price > 10_000.00{
                let new_price = price* 0.05;
                println!("Your discounted price is {:?}",new_price );
            }
        }else {
            println!("Not a valid code");

        }

        println!("Do you want to make another order {} (y/n)",name );
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice");
        let choice = choice.trim().to_uppercase();

        if choice == "N"{
            println!("Thankyou for ordering {} at ICHIRAKU CUSINE",name );
            break 
        }
}
}