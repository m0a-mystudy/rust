fn x_or_y<'a>(x:&str, y:&str) -> &'a str {
    &(x.to_string() + &y)
}
fn main() {
    let c = x_or_y("a", "b");
    println!("{}",c);
}


// struct Circle {
//     x: f64,
//     y: f64,
//     radius: f64,
// }

// impl Circle {
//     fn reference(&self) {
//         println!("taking self by reference!");
//     }
//     fn mutable_reference(&mut self) {
//         println!("taking self by mutable reference!");
//     }
//     fn take_ownership(self) {
//         println!("taking ownership of self!");
//     }
// }

// impl Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * (self.radius * self.radius)
//     }
//     fn glow(&self, increment: f64) -> Circle {
//         Circle {
//             x: self.x,
//             y: self.y,
//             radius: self.radius + increment,
//         }
//     }
// }

// impl Circle {
//     fn new(x: f64, y: f64, radius: f64) -> Circle {
//         Circle {
//             x: x,
//             y: y,
//             radius: radius,
//         }
//     }
// }

// fn main() {
//     let c = Circle::new(0.0,0.0,2.0);
//     println!("{}", c.area());
//     let d = c.glow(2.0).area();
//     println!("{}", d);
// }

// fn main() {
//     let y: &i32;
//     let x = 5;
//     y = &x;

//     println!("{}", y);
// }
// fn main() {
//     let mut x = 5;
//     {
//         let y = &mut x;
//         *y += 1;

//     }
//     println!("{}", x);
// }
// fn foo(v: &Vec<i32>) -> i32 {
//     println!("{}", v[1]);
//     v.push(2);
//     v[1]
// }

// fn main() {
//     let v1 = vec![1, 2, 3];
//     let answer = foo(&v1);
//     println!("{}, {}", v1[0], answer);
// }

// fn main() {
//     // let v = vec![1,2,3];
//     // let v2 =v;
//     // println!("{}\n",v[2])
//     let a = 5;
//     let _y = double(a);
//     println!("{}", a);
// }

// fn double(x: i32) -> i32 {
//     x * 2
// }

// fn main2() {
//     println!("Hello, world!");
//     let x: Option<i32> = Some(42);

//     // println!("hello, {}", match x {
//     //     Some(x) => x,
//     //     None => 0,
//     // });
//     if let Some(x) = x {
//         println!("{}", x);
//     }
// }
