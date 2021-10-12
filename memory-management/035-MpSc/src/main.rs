use std::time::Duration;
use std::sync::mpsc;
use std::thread;

fn main() {
  let (sender, receiver) = mpsc::channel::<i32>();

  thread::spawn(move || {
    for i in 0..5 {
      sender.send(i).unwrap(); // Owner of sender
      thread::sleep(Duration::from_millis(500));
    }
  });

  loop {
    match receiver.recv() { // owner of receiver
      Ok(result) => println!("Received: {}", result),
      Err(_) => {
        println!("Done!");
        break;
      }
    };
  }
}
