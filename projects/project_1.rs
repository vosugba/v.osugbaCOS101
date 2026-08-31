fn main (){
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.00;
	let n:f64 = 5.00;

	//Amount
	let a:f64 = p*((1.00 + (r/100.00)).powf(n));
	println! ("Amount equals to {}",a);

	//Compound interest 
	let ci:f64 = a - p;
	println!("Compound Interest equals to {}",ci);

	println! ("Thus for a principal of {} and with a rate of {} for {}years the Compound Interest is {}",p,r,n,ci);


}