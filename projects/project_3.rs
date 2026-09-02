fn main() {
	let p:f64 = 210_000.000;
	let r:f64 = 5.00;
	let n:f64 = 3.00;

	let a:f64 = p*(1.00-(r/100.00)).powf(n);

	println!("The depreciation of MS.Akudo Ijezie TV set bought for {} depreciating by a rate of {}% for {}years is {:.3}",p,r,n,a );
}