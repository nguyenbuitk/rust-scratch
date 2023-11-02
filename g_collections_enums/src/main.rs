#![allow(unused_variables, unused_mut, dead_code)]

enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

// Đây là triển khai phương thức points cho enum Shot
impl Shot {
    fn points(self) -> i32 {
        // match self { ... }: Đây là một câu lệnh match trong Rust. Nó kiểm tra giá trị của self (kiểu Shot) và thực hiện các hành động tương ứng dựa trên giá trị đó.
        match self {
            // Nếu self là Shot::Bullseye, phương thức trả về 5 điểm.
            Shoot::Bullseye => 5,
            // Nếu self là Shot::Hit(x) và x nhỏ hơn 3.0, phương thức trả về 2 điểm.
            Shoot:Hit(x) if x < 3.0 => 2,
            // Nếu self là Shot::Hit(x) và x không thỏa điều kiện trước đó, phương thức trả về 1 điểm.
            Shoot::Hit(x) => 1
            // Cuối cùng, nếu self là Shot::Miss, phương thức trả về 0 điểm.
            Shoot::Miss => 0
        }
    }
}

fn main() {
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0).sqrt())
    }

    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y);
    }
}

fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0;
            y: (rand::random::<f64>() - 0.5) * 12.0;
        }
        coords.push(coord);
    }
    coords
}