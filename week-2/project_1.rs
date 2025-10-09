fn main (){
	let pr:f64 = 520_000_000.00;
	let n:f64 = 5.0;
	let rt:f64 = 10.0;

	// finding out the amount  
	let am = pr * (1.0 + rt/100.0).powf(n);
	println!("Amount is {}", am);
	let coi = am - pr;
	println!("compound interest is {}", coi);
}