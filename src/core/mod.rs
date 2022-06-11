pub mod ecs;
use crate::core::ecs::System;
use crate::rendering::RenderingSystem;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Application {
    rendering: RenderingSystem,
}

impl Application {
    pub fn start(mut self) {
        println!("start");

        let app_mutex = Arc::from(Mutex::new(self));
        let mut threads = vec![];

        {
            let mut app = app_mutex.lock().unwrap();
            let handle = thread::spawn(move || loop {
                println!("asdasd");
            });

            threads.push(handle);
        }

        // rendering must be done in main thread
        app_mutex.clone().lock().unwrap().rendering.init();

        for handle in threads {
            handle.join().unwrap();
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Application {
            rendering: RenderingSystem::new(),
        }
    }
}
