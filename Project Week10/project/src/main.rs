/* Okay here is my thought process we make a struct (laptops), with brand, quantities and price, next we get the users info of what laptops they want to buy (question said 3 ill just try and make it so they could get multiple ), them make an impl to calculate the laptop price
We could make it that the person could only order 3 laptops by making 3 input variables for the price then probable using the if statment 
Or we find a way to store all the prices in a vector then using a for loop to sum it together and they will most likely be in a loop
Then i will also be making a function to get the input because why not let me practice  
To Avoid stress i left everything as a string object */

fn input_laptop (kini: &str) -> String { 
    println!("{}", kini );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed");
    input.trim().to_uppercase()


}
 struct Laptops {
    brand: String,
    quantity: f64,
    price: f64

    }

 struct Cart {    
 // this struct was done with the aid of AI please note it was NOT COPY AND PASTE I GOT THE CONCEPT FROM CLASS AND I WAS ONLY CONFUSED ON HOW IT COULD BE DONE FOR INSTANCE -> Cart or &mut.
 // All comments were written by me to show my understanding of the code that was written.

    prices : Vec<f64> // making a vector prices which is f64 like we do name: &str

 }

impl Cart{
    fn vec() -> Cart {
        Cart {prices: Vec::new()}  //making the vector empty this was the part i used AI mainly then the add function 

    }

    fn add(&mut self, price: f64) {
        self.prices.push(price);          // functions that pushes the prices to the vector the &mut self makes the cart that was called on mutable and not just calling on it with &self 
    }

    fn total (&self) -> f64 { // function that calculates the total of the prices 
        let mut sum = 0.00;
        for index in 0..self.prices.len() {
            sum += self.prices[index]
        }
        sum
    }
}

fn main () {

   let hp = Laptops{
    brand: String::from("Hp"),
    quantity: 10.00,
    price: 650_000.00
   };

   let ibm = Laptops{
    brand: String::from("IBM"),
    quantity: 6.00,
    price: 755_000.00
   };

   let toshiba = Laptops{
    brand: String::from("Toshiba"),
    quantity: 10.00,
    price: 550_000.00
   };

   let dell = Laptops{
    brand: String::from("Dell"),
    quantity: 4.00,
    price: 850_000.00
   };
   


   println!("WELCOME CUSTOMER TO ALABA ELECTRONICS, HERE THE LIST OF LAPTOPS TO SELL");
   println!("");
   println!("==========================================");
   println!("BRAND         Quantity      Price     CODE");
   println!("HP               10         650_000     H ");
   println!("IBM               6         755_000     I ");
   println!("Toshiba          10         550_000     T ");
   println!("Dell              4         850_000     D ");
   println!("==========================================");
   println!("");
   println!("Please type the code of the brand you want to buy ");


   

 let mut cart = Cart::vec(); //calling the function vec making an empty vector prices 
// like we learned in class this show that cart is now taking ownership of the vector prices 

loop {
    let mut code = input_laptop("Please input the code of the brand you want. Once you are done type DONE");
// This is using the input_laptop function to first of all using the kini (again im sorry for using this but i cant think of any other thing) in the parameter to print the statement then also collecting the input 
    if code =="H"{
        cart.add(hp.price); // this is using the add function above which basically pushes the price into the vector 

    }
    else if code == "I"{
        cart.add(ibm.price);
    }
     else if code == "T"{
        cart.add(toshiba.price);
    }
     else if code == "D"{
        cart.add(dell.price);
    }
     else if code == "DONE"{
       println!("Thanks for ordering ");
       break
    }
    else {
        println!(" The code cannot be found try again please");
    }

}
   println!("The total price: {}",cart.total() ); // this then calls on the total function which basically makes a for loop that in in maakes an empty variable sum which is 0.0 then iterating all the values stored in the price vector it adds them to each other like i have been doing the only thing new here is the "self."

   
}

