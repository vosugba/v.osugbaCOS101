fn main (){
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.00;
	let n:f64 = 5.00;

	//Amount
	let a:f64 = p*(1.00 + (r/100.00).powf(n.into()));
	println! ("Amount equals to {}",a);

	//Compound interest 
	let ci:f64 = a - p;
	println!("Compound Interest equals to {}",ci);


}