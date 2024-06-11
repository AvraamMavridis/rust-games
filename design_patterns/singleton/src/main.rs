use std::sync::Once;
use std::sync::{Arc, Mutex};

struct Singleton {
    data: String,
}

impl Singleton {
    // private constructor
    fn new(data: String) -> Self {
        Singleton { data: data }
    }

    pub fn instance(data: String) -> Arc<Mutex<Self>> {
        static mut SINGLETON: Option<Arc<Mutex<Singleton>>> = None;
        // ensures that a particular initialization is performed only once,
        // even if multiple threads attempt to perform the initialization simultaneously
        static ONCE: Once = Once::new();

        /*
        The operation is considered unsafe because it involves modifying a static mutable variable.
        In Rust, mutable static variables are inherently unsafe due to potential data races and
        undefined behavior when accessed from multiple threads.
        The unsafe block is required to signal that you, as the programmer, are aware of these risks
        and have taken steps to ensure safety.
        In this case, the use of Once ensures that the initialization happens only once,
        which mitigates some of the risks, but Rust still requires the unsafe
        block to acknowledge the potential danger.
        */
        unsafe {
            ONCE.call_once(|| {
                let singleton = Singleton::new(data);

                SINGLETON = Some(Arc::new(Mutex::new(singleton)));
                
            });
            SINGLETON.clone().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton_instance() {
        let instance1 = Singleton::instance("First".to_string());
        let instance2 = Singleton::instance("Second".to_string());
    
        // Both instances should be the same
        assert!(Arc::ptr_eq(&instance1, &instance2));
    
        // The data should be "First" because the singleton is initialized only once
        let data = instance1.lock().unwrap().data.clone();
        assert_eq!(data, "First");
    }
}


