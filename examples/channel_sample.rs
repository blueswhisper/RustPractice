use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 7;

fn main() {
    // 通道有2个端点: `Sender<T>` 和 `Receiver<T>`, 其中`T`是要发送
    // 消息的类型(类型标注是可有可无的)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {
        // sender 发送端可被复制
        let thread_tx = tx.clone();
        // 每个线程都将通过通道来发送它的id
        thread::spawn(move || {
            // 此线程取得`thread_tx`所有权
            // 每个线程都在通道中排队列出消息
            thread_tx.send(id).unwrap();
        });
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // `recv`方法从通道中拿到一个消息,阻塞调用
        ids.push(rx.recv());
    }

    println!("{:?}", ids);
}
