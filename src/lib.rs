use std::sync::mpsc::{Receiver,Sender,channel};
use std::thread::Thread;

pub struct TaskPipe<T: Send> {
    rx: Receiver<T>
}

pub fn source<T: Send, F: FnOnce(Sender<T>)+Send>(task: F) -> TaskPipe<T> {
    let (tx, rx) = channel();

    Thread::spawn(move || {
        task(tx);
    });

    TaskPipe { rx: rx }
}

impl<T: Send> TaskPipe<T> {
    pub fn source<F: FnOnce(Sender<T>)+Send>(self, task: F) -> TaskPipe<T> {
        let (tx, rx) = channel();
        let mut tx2 = tx.clone();

        Thread::spawn(move || {
            for message in self.rx.iter() {
                tx2.send(message);
            };
        });
        Thread::spawn(move || {
            task(tx);
        });

        TaskPipe { rx: rx }
    }

    pub fn pipe<S: Send, F: FnOnce(Receiver<T>, Sender<S>)+Send>(self, task: F) -> TaskPipe<S> {
        let (tx, rx) = channel();

        Thread::spawn(move || {
            task(self.rx, tx);
        });

        TaskPipe { rx: rx }
    }

    pub fn drain<F: FnOnce(Receiver<T>)+Send>(self, task: F) {
        Thread::scoped(move || {
            task(self.rx);
        });
    }
}
