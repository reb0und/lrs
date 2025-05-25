//fn largest<T>(list: &[T]) -> &T {
//    let mut largest = &list[0];
//
//    for num in list {
//        if num > largest {
//            largest = num;
//        }
//    }
//
//    largest
//}

struct Point<T, U> {
    x: T,
    y: U,
}

//impl<T> Point<T> {
//    fn x(&self) -> &T {
//        &self.x
//    }
//}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let i = Point { x: 5, y: 1 };
    let f = Point { x: 1.1, y: 1.2 };
    let f = Point { x: 1, y: 1.2 };

    let num_list = vec![1, 2, 3];

    let p3 = i.mixup(f);

    println!("{}, {}", p3.x, p3.y);

    //let result = largest(&num_list);
    //println!("{result}");

}
