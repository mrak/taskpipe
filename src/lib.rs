use std::sync::mpsc::{Receiver,Sender,channel};
use std::thread::Thread;

pub struct TaskPipe<T: Send> {
    outlet: Receiver<T>
}

impl<T: Send> TaskPipe<T> {
    pub fn tap<F: FnOnce(&mut Sender<T>)+Send>(task: F) -> TaskPipe<T> {
        let (mut tx, rx) = channel();

        Thread::spawn(move || {
            task(&mut tx);
        });

        TaskPipe { outlet: rx }
    }

    pub fn pipe<F: FnOnce(&mut Receiver<T>, &mut Sender<T>)+Send>(mut self, task: F) -> TaskPipe<T> {
        let (mut tx, rx) = channel();

        Thread::spawn(move || {
            task(&mut self.outlet, &mut tx);
        });

        TaskPipe { outlet: rx }
    }

    pub fn drain<F: FnOnce(&mut Receiver<T>)+Send>(mut self, task: F) {
        Thread::spawn(move || {
            task(&mut self.outlet);
        });
    }
}

pub fn pump<T: Send>(rx: &Receiver<T>, tx: &Sender<T>) {
    let tx2 = tx.clone();
    Thread::spawn(move || {
        for message in rx.iter() {
            match tx2.send(message) {
                Err(_) => return,
                Ok(_) => continue
            }
        }
    });
}
