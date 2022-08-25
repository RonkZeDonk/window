use crate::media::ManagerMessage;

/// All valid messages which are sent between threads. Implimentations aren't
/// provided in this module and must be made in the respective threads.
#[derive(Debug, Clone, Copy)]
pub enum ThreadMessage {
    /// Stop the current thread
    Stop,
    /// Echo the message provided
    Echo(&'static str),
    /// Media manager message
    Media(ManagerMessage),
}

/// Thread with a tx and rx channel.
#[derive(Debug)]
pub struct Thread {
    handle: std::thread::JoinHandle<()>,

    tx: crossbeam_channel::Sender<ThreadMessage>,
}

impl Thread {
    /// Create a new thread. The first argument is the `rx` of the thread. Use
    /// this to check for messages directed towards the thread.
    ///
    /// # WARNING
    /// Make sure that you check for the [`ThreadMessage::Stop`] event
    /// otherwise the thread will never be able to end.
    ///
    /// # Example
    /// ```
    /// use window::controller::Thread;
    /// use window::controller::ThreadMessage;
    ///
    /// let thread = Thread::new(move |rx| {
    ///     loop {
    ///         let x = rx.recv().unwrap();
    ///         match x {
    ///             ThreadMessage::Stop => {
    ///                 println!("Stopping Thread");
    ///                 break;
    ///             },
    ///             _ => (),
    ///         };
    ///     }
    /// });
    ///
    /// std::thread::sleep(std::time::Duration::from_secs(1));
    ///
    /// thread.stop();
    /// ```
    #[must_use]
    pub fn new<F>(closure: F) -> Self
    where
        F: FnOnce(crossbeam_channel::Receiver<ThreadMessage>) + Send + 'static,
    {
        let (tx, rx) = crossbeam_channel::unbounded();

        Thread {
            handle: std::thread::spawn(move || {
                closure(rx);
            }),

            tx,
        }
    }

    /// Returns true or false based on if thread is finished executing.
    ///
    /// Wrapper function for [`std::thread::JoinHandle::is_finished()`][is_finished]
    ///
    /// [is_finished]: https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.is_finished
    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }

    /// Join thread to current thread.
    /// Don't use this method to stop a thread. Instead use the `stop` method.
    ///
    /// Once this method is run, the thread is moved and thus can not be used
    /// again.
    ///
    /// Wrapper function for [`std::thread::JoinHandle::join()`][join]
    ///
    /// [join]: https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join
    pub fn join(self) {
        self.handle.join().unwrap()
    }

    /// Send message to thread
    pub fn send_message(&self, message: ThreadMessage) {
        self.tx.send(message).unwrap();
    }

    /// Stop the thread. Once this method is run, the thread is moved and thus
    /// can not be used again.
    pub fn stop(self) {
        self.send_message(ThreadMessage::Stop);
        self.join();
    }
}

/// Thread Controller.
///
/// See `controller::Thread` and `controller::ThreadController::new()`
#[derive(Debug)]
pub struct ThreadController {
    threads: Vec<Thread>,

    rx: crossbeam_channel::Receiver<ThreadMessage>,
}

impl ThreadController {
    /// Create a new controller. `rx` is the reciever from the `tx`, `rx` pair
    /// which was used to create threads.
    ///
    /// # Example
    /// ```
    /// use window::controller::Thread;
    /// use window::controller::ThreadController;
    /// use window::controller::ThreadMessage;
    ///
    /// let (tx, rx) = crossbeam_channel::unbounded();
    ///
    /// let echo = Thread::new(move |rx| {
    ///     loop {
    ///         let x = rx.recv().unwrap();
    ///
    ///         match x {
    ///             ThreadMessage::Stop => {
    ///                 println!("stopping thread");
    ///                 break;
    ///             },
    ///             ThreadMessage::Echo(msg) => {
    ///                 println!("message recieved: {}", msg)
    ///             }
    ///         }
    ///     }
    /// });
    ///
    /// let sender = Thread::new(move |_| {
    ///     std::thread::sleep(std::time::Duration::from_millis(250));
    ///     tx.send(
    ///         ThreadMessage::Echo("Hello there!")
    ///     )
    ///     .unwrap();
    ///     std::thread::sleep(std::time::Duration::from_millis(500));
    ///     tx.send(ThreadMessage::Stop).unwrap();
    ///     std::thread::sleep(std::time::Duration::from_millis(250));
    /// });
    ///
    /// ThreadController::new(rx)
    ///     .add_thread(echo)
    ///     .add_thread(sender)
    ///     .begin();
    /// ```
    #[must_use]
    pub fn new(rx: crossbeam_channel::Receiver<ThreadMessage>) -> Self {
        ThreadController {
            threads: vec![],

            rx,
        }
    }

    /// Add a new thread to the controller
    /// ```
    /// use window::controller::ThreadController;
    /// use window::controller::Thread;
    ///
    /// let (_tx, rx) = crossbeam_channel::unbounded();
    ///
    /// let thread = Thread::new(move |_| {
    ///     println!("hi");
    /// });
    /// let c = ThreadController::new(rx)
    ///     .add_thread(thread);
    /// ```
    pub fn add_thread(mut self, thread: Thread) -> Self {
        self.threads.push(thread);

        self
    }

    /// Returns the length the threads vector
    ///
    /// # Example
    /// ```
    /// use window::controller::Thread;
    /// use window::controller::ThreadController;
    ///
    /// let (_tx, rx) = crossbeam_channel::unbounded();
    ///
    /// let t1 = Thread::new(move |_| {
    ///     println!("hi from thread 1");
    /// });
    /// let t2 = Thread::new(move |_| {
    ///     println!("hi from thread 2");
    /// });
    /// let c = ThreadController::new(rx)
    ///     .add_thread(t1)
    ///     .add_thread(t2);
    ///
    /// assert_eq!(c.threads_count(), 2);
    /// ```
    pub fn threads_count(&self) -> usize {
        self.threads.len()
    }

    /// Join all threads to main.
    ///
    /// # WARNING
    /// Use the `stop_all_threads` method instead of this. This method doesn't
    /// send the stop message to the threads before joining. If you know that
    /// your threads will stop eventually then you can safely use this method.
    ///
    /// The following example will never finish:
    /// ```no_run
    /// use window::controller::ThreadController;
    /// use window::controller::Thread;
    ///
    /// let (_tx, rx) = crossbeam_channel::unbounded();
    ///
    /// let t = Thread::new(move |_| {
    ///     loop { /* forever */ }
    /// });
    /// let c = ThreadController::new(rx)
    ///     .add_thread(t);
    ///
    /// c.join_all_threads();
    /// // Unreachable code
    /// ```
    ///
    /// # Example
    /// ```
    /// use window::controller::ThreadController;
    /// use window::controller::Thread;
    ///
    /// let (_tx, rx) = crossbeam_channel::unbounded();
    ///
    /// let t = Thread::new(move |_| {
    ///     println!("hi");
    /// });
    ///
    /// let c = ThreadController::new(rx)
    ///     .add_thread(t);
    ///
    /// std::thread::sleep(std::time::Duration::from_secs(1));
    /// c.join_all_threads();
    /// ```
    pub fn join_all_threads(mut self) {
        while let Some(thread) = self.threads.pop() {
            thread.join();
        }
    }

    /// Send the stop message to all threads.
    pub fn stop_all_threads(mut self) {
        while let Some(thread) = self.threads.pop() {
            if !thread.is_finished() {
                thread.stop();
            }
        }
    }

    /// Send a message to all threads.
    pub fn send_all(&self, message: ThreadMessage) {
        for thread in &self.threads {
            if !thread.is_finished() {
                thread.send_message(message);
            }
        }
    }

    /// Start the controller's message manager / managing threads
    pub fn begin(self) {
        println!("Started Thread Controller");
        loop {
            let msg = self.rx.recv().unwrap();

            match msg {
                ThreadMessage::Stop => {
                    self.stop_all_threads();
                    break;
                }
                _ => {
                    self.send_all(msg);
                }
            }
        }
    }
}
