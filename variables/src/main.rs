fn main() {

    println!("Mutable variables");
    let mut x = 4;
    println!("X is: {}",x);

    x = 6;
    println!("X is now: {}",x);

    //constants are always immutable
    const MAX_POINTS: u32 = 100000;

    println!("\nShadowing variables");
    let y = 5;
    println!("\tY is {}", y);

    let y = y + 1;
    println!("\tY is now {}",y);

    let y = y * y;
    println!("\tY is now {}",y);


    //Floating-point types
    println!("\n Floating-point types");
    let float_x = 2.0; //f64
    let float_y : f32 = 4.2; //f32



    //Some numeric operations
    println!("\nSome numeric operations");
    let sum = 5+10;
        println!("sum: {}",sum);
    let difference = 95.2-13.2;
        println!("difference: {}",difference);
    let product = 6*8;
        println!("product: {}",product);
    let quotient = 24/2;
        println!("quotient: {}",quotient);
    let remainder = 5%4;
        println!("remainder: {}",remainder);

    //boolean
    let t = true; //implicit
    let f : bool = false; //explicit type annotation
    println!("t is implicitly {}, f is explicitly {}",t,f);

}
