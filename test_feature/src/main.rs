#![allow(dead_code, unused_imports,unused_mut, unused_variables, unused_must_use)]
use std::thread;
use std::sync::mpsc;
// ============= test frame =============
// pub type Frame = Vec<Vec<&'static str>>;
// fn main() {
//     let mut frame: Frame = vec![
//         vec!["X", "O", "X"],
//         vec!["X", "X", "X"],
//         vec!["X", "O", "X"],
//     ];

//     for row in frame {
//         for cell in row {
//             print!("{} ", cell);
//         }
//         println!();
//     }
// }

// ============= test borrow string =============
// fn main() {
//     let mut string_vector: Vec<&str> = Vec::new();

//     // add borrowed string to the vector
//     let string1 = "hello, ";
//     let string2 = "world!";
//     string_vector.push(string1);
//     string_vector.push(string2);

//     for &string in &string_vector {
//         println!("{}", string);
//     }
// }

// ============= not borrow string =============
// fn main() {
//     let mut string_vector: Vec<String> = Vec::new();
    
//     let string1 = String::from("Hello, ");
//     let string2 = String::from("world!");
//     string_vector.push(string1);
//     string_vector.push(string2);

//     for string in &string_vector {
//         println!("{}", string);
//     }
// }

// ============= Ownership =============
// fn main() {
//     let s1 = String::from("abc");
//     let s2 = s1.clone();
//     println!("{}", s1);

//     do_stuff(&s1);
//     println!("{}", s1);
// }

// fn do_stuff(s: &String){

// }

// ============= To iterate through the indices (x, y) of a 2D vector  =============
// fn main() {
//     let values = vec![10, 20, 30];
//     for (index, value) in values.iter().enumerate() {
//         println!("Index: {}, Value: {}", index, value);
//     }
//     // kết quả
//     // Index: 0, Value: 10
//     // Index: 1, Value: 20
//     // Index: 2, Value: 30
// }

fn main() {
    // Tạo một kênh giao tiếp
    let (sender, receiver) = mpsc::channel();

    // Tạo luồng con để gửi dữ liệu
    let sender_thread = thread::spawn(move || {
        sender.send("Hello from sender thread").unwrap();
    });

    // Tạo luồng con để nhận dữ liệu
    let receiver_thread = thread::spawn(move || {
        let data = receiver.recv().unwrap();
        println!("Received: {}", data);
    });

    // Chờ cho cả hai luồng hoàn thành
    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();
}