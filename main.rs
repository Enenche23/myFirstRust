fn main() {
    print_number(40);
    calc(2 , 4);
    // let a:u64;
    // let b:u64;

//    let _a = 2;
//    let _b = 4;
 }

fn print_number ( x :u64) {
    println!("value of x: {}", x);
}

fn calc (a:i64 , b:i64 ) {
    
    // let a = _a as u64;
    // let b = _b as u64;

    let sum = a + b;
    let difference = a - b;
    let product = a * b ;
    let fraction = a / b ;
    let remainder = a % b ;
    println!("The sum, difference, product, fraction, remainder are: {0}, {1}, {2}, {3}, {4}", sum, difference, 
    product, fraction, remainder);
}
