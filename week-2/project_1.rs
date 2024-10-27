fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.0;
	let n:i32 = 5;

	//compound interest
	let a= p*(1.0 +(r / 100.0)).powi(n); 
    println!("Amount is {}", a);
    let cl = a - p;
    println!("Compound Interest is {}", cl );

}

	