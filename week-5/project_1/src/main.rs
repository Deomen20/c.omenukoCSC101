use std::io;

fn main() {
    let P:i32 = 3_200;
    let F:i32 = 3_000;
    let A:i32 = 2_500;
    let E:i32 = 2_000;
    let W:i32 = 2_500;

    println!("Welcome to our lovely restaurant");
    println!("Here is our menu, Please kindly place an order");
    println!("MENU");
    println!("P = Poundo Yam & Edinikaiko Soup - N3_200");
    println!("F = Fried Rice & Chicken - N3_000");
    println!("A = Amala & Ewedu Soup - N2_500");
    println!("E = Eba & Egusi - N2_000");
    println!("W = White Rice & Stew - N2_500");

    // place order
    let mut food = String::new();
    let mut quantity = String::new();
    println!("\nPlease enter your choice of food from the menu given");
    io::stdin().read_line(&mut food).expect("Failed to read input");
    let a:f64= food.trim().parse().expect("Not a valid string");
    println!("Your order is: {}", P/F/A/E/W);

    println!("\nPlease enter the quantity you want from your order");
    io::stdin().read_line(&mut quantity).expect("Not a valid number");
    let b:f64
     = quantity.trim().parse().expect("Not a valid number");
    println!("How much quantity do you need: {}",quantity);

    let mut total_charges:f64 = a * b;


    //total order
    let Total_order = if total_charges > 10_000.00{
        total_charges as f64 * 0.05
    }else {
        total_charges as f64
    };

    println!("\n The total price of your order is N{}", Total_order);
   }
    