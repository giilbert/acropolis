pub mod ecs;
use crate::core::ecs::System;
use crate::rendering::RenderingSystem;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Application {
    rendering: RenderingSystem,
}

impl Application {
    pub fn start(self) {
        println!("start");

        let app_shared = Arc::from(Mutex::new(self));
        let mut threads = vec![];

        {
            let app = app_shared.clone();
            let handle = thread::spawn(move || {
                let mut _app = app.lock().unwrap();
                // TODO
            });

            threads.push(handle);
        }

        // rendering must be done in main thread
        app_shared.clone().lock().unwrap().rendering.init();

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
