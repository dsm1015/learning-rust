
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    
}

impl Rectangle {
    
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn width(&self) -> bool {
        self.width > 0
    }
    
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    //let width1 = 30;
    //let height1 = 50;
    
    //let rect1 = (30,50);
    
    let scale = 1;
    let rect1 = Rectangle{
        width: dbg!(30 * scale),
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45
    };
    
    let sq = Rectangle::square(20);

    dbg!(&rect1);
    
    println!("The rectangle is {:#?}", rect1);
    
    if rect1.width(){
        //println!("The area of the rect1 is {} square pixels.", area(&rect1));
        println!("The area of the rect1 is {} square pixels.", rect1.area());
    }
    
    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold sq?: {}", rect1.can_hold(&sq));
    
}

/*fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}*/
