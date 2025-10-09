fn main(){
	// let identify each item to a variable
	let tosh:f64  = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp :f64 = 750_000.00;
	let dell: f64 =  2_850_000.00;
	let acer:f64 = 250_000.00;
	// representing the quantity as "v"


	// to find the sum of all value  
	let sum:f64 =  tosh + mac + hp + dell + acer;
	println!("the sum of all items is {} ", sum);
	// let average be represnted as "aver"
	let aver:f64 = sum/5.0 ;
	println!("the average of all items is {}", aver );
}
