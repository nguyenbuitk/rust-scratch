// Reference: https://www.koderhq.com/tutorial/rust/concurrency/#what-is
#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

// // I. Simple Thread ==================== ========================================
// fn main() {

//     thread::spawn(|| {
//         for i in 0..10 {
//             println!("Loop 1 iteration: {}", i);
//             // wait a bit before next iteration
//             // for demonstration purposes
//             thread::sleep(Duration::from_millis(300));
//         }
//     });

//     for i in 0..5 {

//         println!("Loop 2 iteration: {}", i);
//         thread::sleep(Duration::from_millis(300));
//     }
//     // The program stops as soon as the main thread completes its loop, so the new thread doesn’t have time to finish. The solution is handles.
//     // output:
//     // Loop 2 iteration: 0
//     // Loop 1 iteration: 0
//     // Loop 2 iteration: 1
//     // Loop 1 iteration: 1
//     // Loop 2 iteration: 2
//     // Loop 1 iteration: 2
//     // Loop 2 iteration: 3
//     // Loop 1 iteration: 3
//     // Loop 2 iteration: 4
//     // Loop 1 iteration: 4
// }



// II. How to join handle ===========================================================
// Handles: Handles là các biến có kiểu dữ liệu std::thread::JoinHandle. Chúng được sử dụng để theo dõi và quản lý các luồng con. Khi chúng ta tạo một luồng con bằng thread::spawn(), nó trả về một handle. Handles này cho phép chúng ta kiểm soát và đợi cho luồng con hoàn thành
// Joining Handles: Khi chúng ta muốn đợi cho một luồng con hoàn thành trước khi tiếp tục chạy chương trình, chúng ta sử dụng phương thức .join() trên handle của luồng con. Nó chặn chương trình chính cho đến khi luồng con hoàn thành.
// Handling Errors: Phương thức .join() có thể ném ra một Result chứa một lỗi nếu luồng con kết thúc với lỗi. Để xử lý lỗi, chúng ta sử dụng .unwrap() để trích xuất giá trị kết quả và xử lý lỗi nếu cần.

// fn main() {
//     // create a thread
//     let handle = thread::spawn(|| {
//         // everything in here runs in its own separate thread
//         for i in 0..10 {
//             println!("Loop 2 iteration: {}", i);
//             thread::sleep(Duration::from_millis(500));
//         }
//     });

//     for i in 0..5 {
//         println!("Loop 1 iteration: {}", i);
//         thread::sleep(Duration::from_millis(300));
//     }
//     handle.join().unwrap();
//     // output 
//     // Loop 2 iteration: 0
//     // Loop 2 iteration: 1
//     // Loop 2 iteration: 2
//     // Loop 2 iteration: 3
//     // Loop 2 iteration: 4
//     // Loop 1 iteration: 0
//     // Loop 1 iteration: 1
//     // Loop 1 iteration: 2
//     // Loop 1 iteration: 3
//     // Loop 1 iteration: 4
// }


// III. How to take ownership from inside a thread  ===========================================================
// when using `move`, The variable will then be taken as something that has ownership inside the thread.
// fn main () {
//     let a:i32 = 5;
//     let handle = thread::spawn(move || {
//         println!("a: {}", a);
//     });
//     handle.join().unwrap();
// }


// IV. How to send message between threads.
// We will use mpsc, stands for Multiple Producer Single Consumer.
// fn main() {
//     // create send/receiver vars
//     // to move data through channel
//     let (tx, rx) = mpsc::channel();
    
//     // Create extra thread that will pass some data to the main thread through the channel
//     thread::spawn(move || {
//         let a:i32 = 5;
//         // send the value of 'a' out of thread
//         tx.send(a).unwrap();
//     });

//     // catch the value with .recv() function
//     let b:i32 = rx.recv().unwrap();
//     println!("Value from a extra thread: {}", b);
// }


// V. How to pass channels to functions
// accept an int32 value from a channel
fn logger(a: mpsc::Sender<i32>) {
    a.send(5).unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {

    });

    handle.join().unwrap();
}