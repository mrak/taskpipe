//! This is a small library that simplifies the process of chaining together several
//! sub-tasks via channels. Each sub-task will be executed in it's own thread and
//! given a read channel as well as a write channel.

use std::sync::mpsc::{Receiver,Sender,channel};
use std::thread::{spawn, JoinHandle};

/// A taskpipe continuation
///
/// This struct is not meant to be used directly. Instead, it the return type of other
/// functions within this library
pub struct TaskPipe<T: 'static + Send> {
    /// The reading end of the previous task's channel
    rx: Receiver<T>
}

/// Begin a taskpipe with a production function
///
/// This is intended for the beginning of a task pipeline. The supplied function
/// will be used to feed the rest of the pipeline.
///
/// ```
/// use std::sync::mpsc::Sender;
///
/// taskpipe::input(|tx: Sender<char>| {
///     for c in "abcdefghijklmnopqrstuvwxyz".chars() {
///         tx.send(c).unwrap();
///     }
/// });
/// ```
pub fn input<T: 'static + Send, F: 'static + FnOnce(Sender<T>)+Send>(task: F) -> TaskPipe<T> {
    let (tx, rx) = channel();

    spawn(move || {
        task(tx);
    });

    TaskPipe { rx: rx }
}

impl<T: Send> TaskPipe<T> {
    /// Insert additional items into the pipeline
    ///
    /// Items from the previous task will be passed through to the next task.
    /// Items produced by the given function will also be received by
    /// downstream tasks.
    ///
    /// ```
    /// use std::sync::mpsc::Sender;
    ///
    /// taskpipe::input(|tx: Sender<char>| {
    ///     for c in "abcdefghijklmnopqrstuvwxyz".chars() { tx.send(c).unwrap(); }
    /// }).input(|tx: Sender<char>| {
    ///     for c in "0123456789".chars() { tx.send(c).unwrap(); }
    /// });
    /// ```
    pub fn input<F: 'static + FnOnce(Sender<T>)+Send>(self, task: F) -> TaskPipe<T> {
        let (tx, rx) = channel();
        let tx2 = tx.clone();

        spawn(move || {
            for message in self.rx.iter() {
                tx2.send(message).unwrap();
            };
        });
        spawn(move || {
            task(tx);
        });

        TaskPipe { rx: rx }
    }

    /// Transform or process pipeline items
    ///
    /// Received upstream task items, process them, and pass them along to
    /// downstream tasks.
    ///
    /// ```
    /// use std::sync::mpsc::{Receiver,Sender};
    ///
    /// taskpipe::input(|tx: Sender<char>| {
    ///     for c in "abcdefghijklmnopqrstuvwxyz".chars() { tx.send(c).unwrap(); }
    /// }).pipe(|rx: Receiver<char>, tx: Sender<bool>| {
    ///     for c in rx.iter() {
    ///         tx.send(c.is_uppercase()).unwrap();
    ///     }
    /// });
    /// ```
    pub fn pipe<S: 'static + Send, F: 'static+FnOnce(Receiver<T>, Sender<S>)+Send>(self, task: F) -> TaskPipe<S> {
        let (tx, rx) = channel();

        spawn(move || {
            task(self.rx, tx);
        });

        TaskPipe { rx: rx }
    }

    /// Finalize the pipeline, consuming any items.
    ///
    /// This method dispatches and returns a Thread JoinGuard while the closure
    /// receives any items that have made their way through the entire
    /// pipeline.
    ///
    /// ## Example
    /// ```
    /// use std::sync::mpsc::{Receiver,Sender};
    ///
    /// let guard = taskpipe::input(|tx: Sender<char>| {
    ///     for c in "abcdefghijklmnopqrstuvwxyz".chars() {
    ///         tx.send(c).unwrap();
    ///     }
    /// }).end(|rx: Receiver<char>| {
    ///     for c in rx.iter() {
    ///         print!("{} ", c);
    ///     }
    /// });
    /// ```
    ///
    /// ## Example with result
    /// ```
    /// use std::sync::mpsc::{Receiver,Sender};
    ///
    /// let result = taskpipe::input(|tx: Sender<char>| {
    ///     for c in "abcdefghijklmnopqrstuvwxyz".chars() { tx.send(c).unwrap(); }
    /// }).end(|rx: Receiver<char>| {
    ///     for c in rx.iter() {
    ///         print!("{} ", c);
    ///     }
    /// }).join();
    /// ```
    // pub fn end<'a, R: Send + 'a, F: 'static + FnOnce(Receiver<T>) -> R + Send>(self, task: F) -> JoinHandle<R> {
    //     spawn (move || {
    //         task(self.rx)
    //     });
    // }
    pub fn end<R: 'static + Send, F: 'static + FnOnce(Receiver<T>) -> R + Send>(self, task: F) -> JoinHandle<R> {
        spawn (move || {
            task(self.rx)
        })
    }
}
