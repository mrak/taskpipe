extern crate taskpipe;

use taskpipe::source;
use std::sync::mpsc::{Receiver,Sender};

#[test]
fn it_works() {
    source(|tx: Sender<char>| {
        for c in "0123456789abcd".chars() {
            tx.send(c);
        }
    }).source(|tx: Sender<char>| {
        tx.send('f');
    }).source(|tx: Sender<char>| {
        tx.send('e');
    }).pipe(|rx: Receiver<char>, tx: Sender<usize>| {
        for c in rx.iter() {
            match c.to_digit(16) {
                Some(u) => tx.send(u),
                None => continue
            };
        }
    }).pipe(|rx: Receiver<usize>, tx: Sender<bool>| {
        for c in rx.iter() {
            print!("{} ", c);
            if c % 2 == 0 {
                tx.send(true);
            } else {
                tx.send(false);
            }
        }
    }).drain(|rx: Receiver<bool>| {
        for c in rx.iter() {
            print!("{} ", c);
        }
    });
}
