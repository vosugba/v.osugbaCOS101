fn main (){
	let _stuff = ["Toshiba","Mac","HP","Dell","Acer"];
	let qty = [2,1,3,3,1];
	let amount = [450_000.00,1_500_000.00,750_000.00,2_850_000.00,250_000.00];

	for i in 0..amount.len(){
	    let total = qty[i] as f64 * amount[i];
        let mut sum = 0.0; 
	    sum += total;

	    println!("{}",sum);
    
        let average = sum / amount.len() as f64;

	    println!("The average for the sales is {}", average );
	}
}

