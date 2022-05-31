#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn check_dimensions(&self) -> bool {
        self.width > 0 && self.height > 0
    }
    
    fn can_it_hold(&self, other_rectangle_instance: &Rectangle) -> bool {
        self.area() > other_rectangle_instance.area()
    }
    
    fn prepare_square(size: u32) ->  Rectangle {
        let the_returned_square = Rectangle {
            width: size,
            height: size,
        };
        return the_returned_square;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };    

    let rect3 = Rectangle {
        width: 100,
        height: 100,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_it_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_it_hold(&rect3));
    
    let main_square = Rectangle::prepare_square(10);
    println!("Square is prepared {:#?}", main_square);
    println!("Perimeter of the square is {}", main_square.perimeter());
}

