
struct rect {
    width: u32,
    height: u32
}

struct square {
    side: u32,
}


impl rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn shape() -> String {
        String::from("rectangle")
    }

}

impl square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
    fn shape() -> String {
        String::from("square")
    }
}

fn main() {
 let rectangle = rect {width: 3, height: 4};
 let square = square {side: 4};

 println!("The area of the {} is {}", rect::shape(), rectangle.area());
 println!("The area of the {} is {}", square::shape(), square.area());
}
