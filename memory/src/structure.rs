enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

enum Direction {
    North,
    East,
    South,
    West,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in: u64,
}

struct rectangle {
    width: u32,
    height: u32,
}

impl rectangle {
    fn area(&self) -> u32 {
        self.width * self.height;
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn calculate_area(shape: Shape) -> f64 {
    let ans:f64 = match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    };
    return ans
}

fn main() {
    let sidharth = User {
        active: true,
        name: "Sidharth",
        email: "philkhanasidharth14@gmail.com",
        sign_in: 0,
    };

    //implementing structs
    let rect1 = rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = rectangle {
        width: 10,
        height: 40,
    };
    println!("The area of the rectangle is {}", rect1.area());
    println!("The perimeter of the rectangle is {}", rect1.perimeter());

    let my_direction = Direction::East;
    match my_direction {
        Direction::North => println!("Moving North"),
        Direction::East => println!("Moving East"),
        Direction::South => println!("Moving South"),
        Direction::West => println!("Moving West"),
    }

    let my_shape = Shape::Circle(3.14);
    let my_rectangle = Shape::Rectangle(10.0, 20.0);
    let my_square = Shape::Square(10.0);

    let area = calculate_area(my_shape);
    let rect_area = calculate_area(my_rectangle);
    let square_area = calculate_area(my_square);
    println!("The area of the shape is {}", area);
    println!("The area of the rectangle is {}", rect_area);
    println!("The area of the square is {}", square_area);



    println!("User {} has email {}", sidharth.name, sidharth.email);
}
