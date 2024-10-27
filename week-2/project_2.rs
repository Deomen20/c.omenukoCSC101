fn main() {
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let acer:f64 = 250_000.00;
	let s = t + m + h + d + acer;
	println!("Sum is {}", s);
	let a = t + m + h + d + acer / 5.0;
	println!("Average is {}", a);

}