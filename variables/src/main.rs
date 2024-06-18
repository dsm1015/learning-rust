//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is {}", x);
    }
    println!("The value of x is {}", x);
    
    let spaces = "    ";
    let spaces = spaces.len();
    println!("len of spaces: {}", spaces);
    
    //tuples
    let tup: (i32, f64, u8) = (500,6.4,1);
    
    // destructure tuple value
    let (x, y, z) = tup;
    println!("The value of y is {y}");
    
    let five_hundred = tup.0;
    println!("The value of five_hundred is {five_hundred}");
    
    //arrays
    let a:[i32; 5]  = [1,2,3,4,5];
    let b = [3; 5]; // [3,3,3,3,3]
    
    let first = a[0];
    println!("The value of first is {first}");
}