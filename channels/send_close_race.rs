#![feature(mpmc_channel)]
#![allow(unused)]

use std::{
    sync::{
        mpmc::{self, Receiver, Sender},
        mpsc,
    },
    thread,
    time::Duration,
};

type Chan<T> = (Sender<T>, Receiver<T>);

fn fibonacci(n: i32, c: Chan<i32>) {
    let mut x = 0;
    let mut y = 1;
    for i in 0..n {
        c.0.send(x).unwrap();
        let old_x = x;
        x = y;
        y = old_x + y;
    }
    //println!("fib received: {:?}", c.1.recv());
    drop(c);
    //println!("fib received: {:?}", c.1.recv());
    println!("exit fibonacci")
}

fn asdf(n: i32, c: Sender<i32>) {
    thread::sleep(Duration::from_millis(2000));
    dbg!(c.send(n)).unwrap();
}

fn main() {
    let c = mpmc::sync_channel(0);

    let c1 = c.clone();
    thread::spawn(move || fibonacci(10, c1));
    thread::spawn(move || asdf(12345, c.0));

    for i in c.1 {
        thread::sleep(Duration::from_millis(100));
        println!("main received: {i}")
    }

    thread::sleep(Duration::from_millis(2000));

    println!("exit main")
}
