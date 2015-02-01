extern crate taskpipe;

use std::sync::mpsc::{Receiver,Sender};

#[test]
fn it_inputs_transforms_and_joins_to_main_all_items() {
    taskpipe::input(|tx: Sender<char>| {
        for c in "0123456789abcd".chars() {
            tx.send(c);
        }
    }).input(|tx: Sender<char>| {
        tx.send('f');
    }).input(|tx: Sender<char>| {
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
            if c % 2 == 0 {
                tx.send(true);
            } else {
                tx.send(false);
            }
        }
    }).join(|rx: Receiver<bool>| {
        let mut count = 0;
        for c in rx.iter() {
            count = count + 1;
        }
        assert_eq!(count, 16);
    });
}
