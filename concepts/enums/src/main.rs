#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    //...
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::Alabama);
    value_in_cents(penny);
    value_in_cents(quarter);
    
    if let Coin::Quarter(state) = nickel {
        println!("Quarter from: {:?}!", state);
    } else {
        println!("Not a quarter!");
    }
}


fn value_in_cents(coin: Coin) -> u32 {
    match coin { 
        Coin::Penny => {
            println!("Ooh a lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}