fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(20 * scale),
        height: 50,
    };

    dbg!(&rect1);
    println!("The area of the rectangle is {}", &rect1.area());

    if rect1.width() {
        println!("The rectangle has a non-zero width of {}", &rect1.width);
    }
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 20,
        height: 60,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(30);
    if sq.height() {
        println!("This is a non-zero square with width: {}", sq.width);
    }
    
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    // fn area(& mut self) -> u32 {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }

    // associated function; Rectangle::square(size); no &self
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

impl Rectangle {
    fn height(&self) -> bool {
        self.height > 0
    }
}