fn main() {
	let ta:f64 = 450_000.00 ;
	let ma:f64 = 1_500_000.00;
	let ha:f64 = 750_000.00;
	let da:f64 = 2_850_000.00;
	let aa:f64 = 250_000.00 ;
	let tq:f64 = 2.00;
	let mq:f64 = 1.00;
	let hq:f64 = 3.00;
	let dq:f64 = 3.00;
	let aq:f64 = 1.00;

	let sum:f64 = (ta*tq)+(ma*mq)+(ha*hq)+(da*dq)+(aa*aq);
    let qty:f64 = tq+mq+hq+dq+aq;

    let avg:f64 = sum/qty;

    println! ("The average of the sum total of sales {:.5} and for a total quantity of {:.5} of P.M. Okeke and Sons Ltd is {}",sum,qty,avg);



}