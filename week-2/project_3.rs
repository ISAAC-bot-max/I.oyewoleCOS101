fn main(){
	let val:f64 = 510_000.00;
	let dep:f64 = 5.0;
	let yrs: f64 = 3.0;

	// fiding the value of the car after depreciation
	let a = val * (1.0 - yrs/100.0).powf(dep);
	println!("the value of the tv  is {}", a );

}