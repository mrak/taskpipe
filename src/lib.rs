//! This is a small library that simplifies the process of chaining together several
//! sub-tasks via channels. Each sub-task will be executed in it's own thread and
//! given a read channel as well as a write channel.

use std::sync::mpsc::{Receiver,Sender,channel};
use std::thread::Thread;

/// A taskpipe continuation
///
/// This struct is not meant to be used directly. Instead, it the return type of other
/// functions within this library
pub struct TaskPipe<T: Send> {
    /// The reading end of the previous task's channel
    rx: Receiver<T>
}

/// Begin a taskpipe with a production function
///
/// This is intended for the beginning of a task pipeline. The supplied function
/// will be used to feed the rest of the pipeline.
/// ```
/// taskpipe::input(|tx: Sender<char>| {
///     for c in "abcdefghijklmnopqrstuvwxyz".chars() {
///         tx.send(c);
///     }
/// });
/// ```
pub fn input<T: Send, F: FnOnce(Sender<T>)+Send>(task: F) -> TaskPipe<T> {
    let (tx, rx) = channel();

    Thread::spawn(move || {
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
    /// ```
    /// taskpipe::input(|tx: Sender<char>| {
    ///     for c in "abcdefghijklmnopqrstuvwxyz".chars() { tx.send(c); }
    /// }).input(|tx: Sender<char>| {
    ///     for c in "0123456789".chars() { tx.send(c); }
    /// });
    /// ```
    pub fn input<F: FnOnce(Sender<T>)+Send>(self, task: F) -> TaskPipe<T> {
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

    /// Transform or process pipeline items
    ///
    /// Received upstream task items, process them, and pass them along to
    /// downstream tasks.
    /// ```
    /// taskpipe::input(|tx: Sender<char>| {
    ///     for c in "abcdefghijklmnopqrstuvwxyz".chars() { tx.send(c); }
    /// }).pipe(|rx: Receiver<usize>, tx: Sender<bool>| {
    ///     for c in rx.iter() {
    ///         tx.send(c.is_uppercase());
    ///     }
    /// });
    /// ```
    pub fn pipe<S: Send, F: FnOnce(Receiver<T>, Sender<S>)+Send>(self, task: F) -> TaskPipe<S> {
        let (tx, rx) = channel();

        Thread::spawn(move || {
            task(self.rx, tx);
        });

        TaskPipe { rx: rx }
    }

    /// Finalize the pipeline, consuming any items.
    ///
    /// This method dispatches and joins the final task to the parent's thread,
    /// while receiving any items that have made their way through the entire
    /// pipeline.
    /// ```
    /// taskpipe::input(|tx: Sender<char>| {
    ///     for c in "abcdefghijklmnopqrstuvwxyz".chars() { tx.send(c); }
    /// }).output(|tx: Sender<char>| {
    ///     for c in rx.iter() {
    ///         print!("{} ", c);
    ///     }
    /// });
    /// ```
    pub fn end<F: FnOnce(Receiver<T>)+Send>(self, task: F) {
        Thread::scoped(move || {
            task(self.rx);
        });
    }

    /// Finalize the pipeline without a thread, consuming any items.
    ///
    /// This method executes it's closure in the current thread without
    /// spawning a sub-process.
    /// ```
    /// taskpipe::input(|tx: Sender<char>| {
    ///     for c in "abcdefghijklmnopqrstuvwxyz".chars() { tx.send(c); }
    /// }).join(|tx: Sender<char>| {
    ///     for c in rx.iter() {
    ///         print!("{} ", c);
    ///     }
    /// });
    /// ```
    pub fn join<F: Fn(Receiver<T>)>(self, task: F) {
        task(self.rx);
    }
}
