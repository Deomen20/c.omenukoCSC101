fn main() {
	let p:f64 = 210_000.00;
    let r:f64 = 5.0;
    let n:i32 = 3;

    //compound interest for depreciation
    let a = p*(1.0 - (r/100.00)).powi(n);
    println!("Amount is {}", a);
    let cl = p - a;
    println!("Compound Interest is {}", cl);

}