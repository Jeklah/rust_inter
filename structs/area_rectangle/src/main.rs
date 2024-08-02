#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect2 is {:#?}", rect2);
    println!("rect2 is {:?}", rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    let scale = 2;
    let rect4 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect4);

    let rect5 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect5.width() {
        println!("The rectangle has a nonzero width; it is {}", rect5.width);
    }

    let rect6 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect7 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect8 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect6 hold rect7? {}", rect6.can_hold(&rect7));
    println!("Can rect6 hold rect8? {}", rect6.can_hold(&rect8));

    let mut r = Rectangle {
        width: 1,
        height: 2,
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    r.set_width(3);
    Rectangle::set_width(&mut r, 2);

    let mut rect9 = Rectangle {
        width: 0,
        height: 0,
    };

    println!("{}", rect9.area());

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };
    let max_rect = rect9.max(other_rect);
    rect9.set_width(1);
    let rect9_ref = &rect9;

    let mut rect10 = Rectangle {
        width: 0,
        height: 1,
    };
    let other_rect10 = Rectangle {
        width: 1,
        height: 0,
    };
    rect10.set_to_max(other_rect10);

    // rect9_ref.set_width(5);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
