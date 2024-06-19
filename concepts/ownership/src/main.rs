
fn main() {
    let mut s1 = String::from("hello");
    
    let len = calculate_length_borrow(&s1);
    
    println!("The length of '{s1}' is {len}");

    calculate_length_mutable_reference(&mut s1);
    
    let len = calculate_length_borrow(&s1);
    
    println!("The length of '{s1}' is {len}");
    
    mutable_with_immutable();
    
    let s1_literal = "goodbye world";
    let first = first_word(&s1[..]); // slicing lets me use String type
    let first_literal = first_word(&s1_literal[..]); // str type
    
    //s1.clear(); Error when we try to mutate something being borrowed.
    
    println!("s1 First Word: {first}");
    println!("bye First Word: {first_literal}");
    
}

// borrowing (read only)
fn calculate_length_borrow(s: &String) -> usize {
    s.len()
}

// mutable reference (can change value)
fn calculate_length_mutable_reference(s: &mut String) {
    s.push_str(", world");
}

fn mutable_with_immutable() {
    let mut s = String::from("Hello");
    
    let r1 = &s;
    let r2 = &s;
    
    //let r3 = &mut s; //cannot borrow as mutable here because it is being borrowed as immutable
    
    println!("{r1} and {r2}");
    
    let r3 = &mut s; //I can borrow here because r1 and r2 are no longer used
    r3.push_str(", world");
    println!("{r3}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}

