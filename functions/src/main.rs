
fn main() {
    println!("Hello, world!");
    
    let x: i32 = {
        let y = 3; // statement (has a semicolon ;)
        y + 1 // expression (returns as  value, no semicolon)
    };
    
    another_function(x);
    
    print_labeled_measurements(x,'h');
    
    let z = five();
    println!("The value of z is {z}");
    
    let z = plus_one(z);
    println!("The value of z is {z}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}