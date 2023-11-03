#![allow(unused_variables, unused_mut, dead_code)]

fn main() {
    let x = 5;

    // Closure không tham số, in giá trị biến x
    let print_x = || {
        println!("x is: {}", x);
    };

    // Closure có tham số, cộng thêm một giá trị
    let add_y = |y| {
        x + y
    };

    // Sử dụng closure
    print_x(); // In ra: x is: 5
    let result = add_y(10); // result = 15
}
