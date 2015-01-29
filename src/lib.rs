use std::sync::mpsc::{Receiver,Sender,channel};
use std::thread::Thread;

pub struct TaskPipe<T: Send> {
    outlet: Receiver<T>
}

pub fn tap<T: Send, F: FnOnce(&Sender<T>)+Send>(task: F) -> TaskPipe<T> {
    let (tx, rx) = channel();

    Thread::spawn(move || {
        task(&tx);
    });

    TaskPipe { outlet: rx }
}

impl<T: Send> TaskPipe<T> {
    pub fn pipe<S: Send, F: FnOnce(Receiver<T>, Sender<S>)+Send>(self, task: F) -> TaskPipe<S> {
        let (tx, rx) = channel();

        Thread::spawn(move || {
            task(self.outlet, tx);
        });

        TaskPipe { outlet: rx }
    }

    pub fn drain<F: FnOnce(Receiver<T>)+Send>(self, task: F) {
        Thread::scoped(move || {
            task(self.outlet);
        });
    }
}

pub fn pump<T: Send>(rx: Receiver<T>, tx: &Sender<T>) {
    let mut tx2 = tx.clone();

    Thread::spawn(move || {
        for message in rx.iter() {
            tx2.send(message);
        };
    });
}
