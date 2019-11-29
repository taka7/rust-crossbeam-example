use crossbeam::queue::ArrayQueue;
use std::sync::Arc;

fn main() {
    let q = Arc::new(ArrayQueue::new(16));

    let q_clone = q.clone();
    let handler = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(q_clone.pop(), Ok(3.0));
        assert_eq!(q_clone.pop(), Ok(4.2));
        println!("{:?}", q_clone.pop());
        32
    });

    std::thread::sleep(std::time::Duration::from_millis(10));

    assert_eq!(q.push(3.0), Ok(()));
    assert_eq!(q.push(4.2), Ok(()));

    println!("{:?}", handler.join());
    println!("Hello, world!");
}
