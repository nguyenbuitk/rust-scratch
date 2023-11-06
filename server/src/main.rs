fn main() {
  let string = String::from("127.0.0.1:8080");
  let string_slice = &string[10..14];  

  // debug macro will call some function underneath
  dbg!(&string);
  dbg!(string_slice);
  // let server = Server::new("127.0.0.1:8080");
  // server.run();
}





fn do_stuff(s: String) {

}

fn do_stuff_2(s: &str){
  
}

struct Server {
  addr: String,
}

impl Server {
  fn new(addr: String) -> Self {
    Self {
      addr
    }
  }

  // nếu truyền vào (self), nó sẽ làm ownership của variable và bị deallocate khi hàm run kết thúc. Để tránh việc này, sử dụng cơ chế borrow
  fn run (&mut self) {

  }
}
