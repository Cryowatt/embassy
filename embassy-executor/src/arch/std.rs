#[cfg(feature = "executor-interrupt")]
compile_error!("`executor-interrupt` is not supported with `arch-std`.");

#[cfg(feature = "executor-thread")]
pub use thread::*;
#[cfg(feature = "executor-thread")]
mod thread {
    use std::sync::{Condvar, Mutex};

    #[cfg(feature = "nightly")]
    pub use embassy_macros::main_std as main;

    use crate::raw::PenderContext;
    use crate::thread::ThreadContext;

    /// TODO
    // Name pending
    pub struct Context {
        signaler: &'static Signaler,
    }

    impl Default for Context {
        fn default() -> Self {
            Self {
                signaler: &*Box::leak(Box::new(Signaler::new())),
            }
        }
    }

    impl ThreadContext for Context {
        fn context(&self) -> PenderContext {
            unsafe { core::mem::transmute(self.signaler) }
        }

        fn wait(&mut self) {
            self.signaler.wait()
        }
    }

    #[export_name = "__pender"]
    fn __pender(context: PenderContext) {
        let signaler: &'static Signaler = unsafe { std::mem::transmute(context) };
        signaler.signal()
    }

    struct Signaler {
        mutex: Mutex<bool>,
        condvar: Condvar,
    }

    impl Signaler {
        fn new() -> Self {
            Self {
                mutex: Mutex::new(false),
                condvar: Condvar::new(),
            }
        }

        fn wait(&self) {
            let mut signaled = self.mutex.lock().unwrap();
            while !*signaled {
                signaled = self.condvar.wait(signaled).unwrap();
            }
            *signaled = false;
        }

        fn signal(&self) {
            let mut signaled = self.mutex.lock().unwrap();
            *signaled = true;
            self.condvar.notify_one();
        }
    }

    /// TODO
    // Type alias for backwards compatibility
    pub type Executor = crate::thread::ThreadModeExecutor<Context>;
}
