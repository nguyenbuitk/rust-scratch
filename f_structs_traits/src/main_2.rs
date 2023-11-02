trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    let circle = Circle {radius: 5.0};
    let square = Square {side: 4.0};

    print_area(&circle);
    print_area(&square);
}

// shape: tham số của hàm
// &impl Shape cho tham số shape cho phép bạn truyền vào bất kỳ đối tượng nào thỏa mãn trait Shape, không cần biết cụ thể loại đối tượng đó là gì. 
// Trong &impl Shape, ký tự & đại diện cho một tham chiếu, chấp nhận một tham chiếu đến bất kỳ đối tượng nào implement trait Shape. Ký tự & trong trường hợp này là tham chiếu vô hướng (immutable reference), cho phép bạn tham chiếu đến dữ liệu một cách không thay đổi (immutable) trong hàm print_area. Điều này đồng nghĩa với việc bạn có quyền đọc (read) dữ liệu của đối tượng thông qua tham chiếu, nhưng không có quyền sửa đổi (modify) dữ liệu đó.
// Nếu bạn không sử dụng ký tự &, ví dụ: impl Shape, thì nghĩa là tham số shape không phải là một tham chiếu mà là trực tiếp một giá trị hoặc đối tượng kiểu implement trait Shape. Trong trường hợp này, Rust sẽ thực hiện một "chuyển đổi sở hữu" (ownership transfer) khi bạn gọi hàm print_area. Điều này có nghĩa rằng bạn chuyển quyền sở hữu của đối tượng tới hàm print_area, và sau khi hàm kết thúc, bạn không thể truy cập đối tượng đó nữa trong phạm vi của hàm gọi. Chuyển đổi sở hữu này có thể không phù hợp nếu bạn muốn tiếp tục sử dụng đối tượng sau khi gọi hàm print_area.
// Vì vậy, việc sử dụng &impl Shape cho phép bạn truyền tham chiếu đến đối tượng, giữ nguyên quyền sở hữu của đối tượng gốc, và cho phép bạn tiếp tục làm việc với nó sau khi gọi hàm. Tuy nhiên, bạn cũng có thể sử dụng impl Shape nếu bạn thực sự muốn chuyển đổi sở hữu của đối tượng.
fn print_area(shape: &impl Shape) {
    println!("Area: {}", shape.area());
}