// Goal: print the lyrics to "The Twelve Days of Christmas" using loops
fn main() {
    
    // array of size 12 for each day
    let twelve_days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let twelve_gifts = [
        "a partridge in a pear tree!",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings!",
        "Six geese a layin'",
        "Seven swans a swimmin'",
        "Nine ladies dancin'",
        "Ten lords a leapin'",
        "Eleven pipers pipin'",
        "Eight maids milkin'",
        "Twelve drummers drummin'"
    ];
    
    // on the nth day of Christmas...
    for day in 0..12 { 
        println!("On the {} of Christmas\nMy true love sent to me", twelve_days[day]);
        for gift in (0..=day).rev() {
            // if first day
            if day > 0 && gift == 0 {
                println!("and {}", twelve_gifts[gift]);
            } else {
                println!("{}", twelve_gifts[gift]);
            }
        }
        println!();
    }
}
