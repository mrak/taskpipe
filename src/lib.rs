use std::sync::mpsc::{Receiver,Sender,channel};
use std::io::{ChanReader,ChanWriter};
use std::thread::Thread;

struct Pipe<T: Send> {
    rx: Receiver<T>
}

impl<T: Send> Pipe<T> {
    fn tap<F: FnOnce(&mut Sender<T>)+Send>(task: F) -> Pipe<T> {
        let (mut tx, rx) = channel();

        Thread::spawn(move || {
            task(&mut tx);
        });

        Pipe { rx: rx }
    }

    fn drain<F: FnOnce(&mut Receiver<T>)+Send>(mut self, task: F) {
        Thread::spawn(move || {
            task(&mut self.rx);
        });
    }

    fn pipe<F: FnOnce(&mut Receiver<T>, &mut Sender<T>)+Send>(mut self, task: F) -> Pipe<T> {
        let (mut tx, rx) = channel();

        Thread::spawn(move || {
            task(&mut self.rx, &mut tx);
        });

        Pipe { rx: rx }
    }
}
