use lection_3::point::point2d::Point2d;
use lection_3::point::point3d::Point3d;

fn main() {
    let mut p2d: Point2d = Point2d::new(10, 15);
    let p3d: Point3d = Point3d::new(1, 3, 2);

    print_length(point: &p2d);
    print_length(point: &p3d);

    let o: Option<Point2d> = Some(Point2d::new(1, 2));
    let r: Result<Point3d, String> = Result::Ok(Point3d::new(1, 2, 2));

    match r {
        Ok(_) => todo!(),
        Err(_) => println!("Error"),
    }

    match o {
        Some(Point2d { x, y }) => {
            println!("{} {}", x, y);
        }
        None => println!("None"),
    }

    let i = 0i32;

    match i {
        0 => println!("zero"),
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }

    if let 5 = i {
        println!("i = 5");
    }

    // if let Some(q: Point2d) = o {
    //     println!("q = {:?}", q);
    // }

    println!("{}", point.add());

    println!("{}", point.add());
}

fn print_length(point: &dyn Point)  {
    println!("{}", point.length());
}

fn match_point(point: &Point) {
    match point {
        &Point::P2d(Point2d { ref x, ref y}) => todo!(),
        &Point::P3d(Point3d { ref x, ref y, ref z}) => todo!(),
    }
}

// enum Point {
//     P2d(Point2d),
//     P3d(Point3d),
// }
