
enum Shape {
    Triangle,
    Square,
}
trait Shape_Trait {
    fn get_shape() -> Shape;
    fn get_value(self) -> i32;
}

struct Triangle {
    hight:i32,
    base:i32
}

impl Shape_Trait for Triangle {
    fn get_shape() -> Shape {
        Shape::Triangle
    }
    fn get_value(self) -> i32 {
        (self.hight*self.base)/2
    }
}

struct Square {
    wide: i32,
    hight:i32
}

impl Shape_Trait for Square{
    fn get_shape() -> Shape {
        Shape::Square
    }
    fn get_value(self) -> i32 {
        self.wide*self.hight
    }
}

fn calculate_area<T: Shape_Trait>(object: T) -> i32 {
    let len = object.get_value()as i32;
    match <T as Shape_Trait>::get_shape() {
        Shape::Square => len,
        Shape::Triangle => len/2,
    }
}

fn main() {
    let triangle = Triangle{ hight: 100,base: 20 };
    let square = Square { wide: 20,hight:10 };

    println!(
        "The area of triange is ：{} 
        and the area of square is：{}",
        calculate_area(triangle),
        calculate_area(square)
    );
}