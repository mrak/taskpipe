extern crate taskpipe;

use std::sync::mpsc::{Receiver,Sender};

#[test]
fn it_inputs_transforms_and_joins_to_main_all_items() {
    let result = taskpipe::input(|tx: Sender<char>| {
        // send radix-16 characters
        for c in "0123456789abcd".chars() {
            tx.send(c).unwrap();
        }
    }).input(|tx: Sender<char>| {
        // add an 'f'
        tx.send('f').unwrap();
    }).input(|tx: Sender<char>| {
        // add an 'e'
        tx.send('e').unwrap();
    }).pipe(|rx: Receiver<char>, tx: Sender<u32>| {
        // Convert radix-16 characters to numbers
        for c in rx.iter() {
            match c.to_digit(16) {
                Some(u) => tx.send(u).unwrap(),
                None => continue
            };
        }
    }).pipe(|rx: Receiver<u32>, tx: Sender<bool>| {
        // Even number detector
        for c in rx.iter() {
            if c % 2 == 0 {
                tx.send(true).unwrap();
            } else {
                tx.send(false).unwrap();
            }
        }
    }).end(|rx: Receiver<bool>| {
        // Count 'True'
        let mut count = 0;
        for c in rx.iter() {
            if c {
                count += 1;
            }
        }
        count
    }).join();

    assert_eq!(result.ok().unwrap(), 8);
}
