#![allow(unused_variables, unused_mut, dead_code)]

// details of enum https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

// Tọa độ 2 chiều
// derive là một attribute trong Rust, nó cho phép kiểu Coord được đánh dấu để tự động triển khai phương thức Debug cho việc in ra giá trị của kiểu này để kiểm tra (debug).
#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

// Đây là triển khai phương thức points cho enum Shot
impl Shot {
    fn points(self) -> i32 {
        // match self { ... }: Đây là một câu lệnh match trong Rust. Nó kiểm tra giá trị của self (kiểu Shot) và thực hiện các hành động tương ứng dựa trên giá trị đó.
        match self {
            // Nếu self là Shot::Bullseye, phương thức trả về 5 điểm.
            Shot::Bullseye => 5,
            // Nếu self là Shot::Hit(x) và x nhỏ hơn 3.0, phương thức trả về 2 điểm.
            Shot::Hit(x) if x < 3.0 => 2,
            // Nếu self là Shot::Hit(x) và x không thỏa điều kiện trước đó, phương thức trả về 1 điểm.
            Shot::Hit(x) => 1,
            // Cuối cùng, nếu self là Shot::Miss, phương thức trả về 0 điểm.
            Shot::Miss => 0
        }

        // similar python code:
        // if self.shot_type == "Bullseye":
        //     return 5
        // elif self.shot_type == "Hit" and self.distance is not None:
        //     if self.distance < 3.0:
        //         return 2
        //     else:
        //         return 1
        // else:
        //     return 0

    }
}

fn main() {
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    for coord in arrow_coords {
        coord.print_description();
        // Dòng này sử dụng câu lệnh match để kiểm tra khoảng cách từ tâm của tọa độ (được tính bởi coord.distance_from_center()) và dựa vào giá trị đó để xác định kiểu bắn (Shot).
        // Nếu khoảng cách nhỏ hơn 1.0 (x if x < 1.0), thì tạo một Shot::Bullseye 
        // Nếu khoảng cách nằm trong khoảng từ 1.0 đến 5.0 (x if x < 5.0), thì tạo một Shot::Hit(x) với giá trị khoảng cách x.
        // Trong trường hợp còn lại (khoảng cách lớn hơn hoặc bằng 5.0), tạo một Shot::Miss (bắn không trúng).

        // Câu lệnh match yêu cầu sử dụng biến để trích xuất giá trị và kiểm tra điều kiện trong mẫu mô hình hoá, không phải trong cách sử dụng if.
        let shot = match coord.distance_from_center() {
            x if x < 1.0 => Shot::Bullseye, // if coord.distance_from_center < 1.0
            x if x < 5.0 => Shot::Hit(x),
            _ => Shot::Miss, 
        };
        shots.push(shot);
    }

    let mut total = 0;
    for shot in shots {
        total += shot.points();
    }

    println!("Final point total is: {}", total)
}

// implement một số function cho Coord 
impl Coord {
    // tính khoảng cách từ tọa độ đến tâm
    fn distance_from_center(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0).sqrt()
    }

    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y
        );
    }
}

fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            // rand::random::<f64>() là một phương thức trong thư viện Rust để tạo ra một số thực kiểu f64 ngẫu nhiên trong khoảng từ 0.0 đến 1.0.
            // rand::random::<f64>() - 0.5 giúp dịch chuyển giá trị ngẫu nhiên từ khoảng 0.0 - 1.0 về khoảng -0.5 đến 0.5.
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}