use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Philosopher {
    id: usize,
    left: Arc<Mutex<Chopstick>>,
    right: Arc<Mutex<Chopstick>>,
}

impl Philosopher {
    fn new(id: usize, left: Arc<Mutex<Chopstick>>, right: Arc<Mutex<Chopstick>>) -> Self {
        Philosopher { id, left, right }
    }

    fn think(&self) {
        println!("Philosopher {}: Thinking...", self.id);
        thread::sleep(Duration::from_secs(2));
    }

    fn eat(&self) {
        let mut left_cs = if let Ok(chopstick) = self.left.try_lock() {
            chopstick
        } else {
            println!("Philosopher {}: OOPS! Left chopstick not available. Waiting...", self.id);
            self.left.lock().unwrap()
        };
        left_cs.get(self);

        let mut right_cs = if let Ok(chopstick) = self.right.try_lock() {
            chopstick
        } else {
            println!("Philosopher {}: OOPS! Right chopstick not available. Waiting...", self.id);
            self.right.lock().unwrap()
        };
        right_cs.get(self);

        println!("Philosopher {}: Eating...", self.id);
        thread::sleep(Duration::from_secs(4));

        right_cs.put(self);
        left_cs.put(self);
    }
}

struct Chopstick {
    id: usize,
    by: Option<usize>,
}

impl Chopstick {
    fn new(n: usize) -> Self {
        Chopstick {
            id: n,
            by: None,
        }
    }

    fn get(&mut self, p: &Philosopher) {
        if self.by != None {
            panic!(); // Invalid state. Mutex is not working?
        }

        println!("Chopstick {}: Picked up by Philosopher {}", self.id, p.id);
        self.by = Some(p.id);
    }

    fn put(&mut self, p: &Philosopher) {
        if self.by != Some(p.id) {
            panic!(); // Invalid state. Mutex is not working?
        }

        println!("Chopstick {}: Released by Philosopher {}", self.id, p.id);
        self.by = None;
    }
}

fn deploy(n: usize, for_left: Arc<Mutex<Chopstick>>, for_right: Arc<Mutex<Chopstick>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let ph = Philosopher::new(n, for_left, for_right);
        loop {
            ph.think();
            ph.eat();
        }
    })
}

fn main() {
    let cs1 = Arc::new(Mutex::new(Chopstick::new(1)));
    let cs2 = Arc::new(Mutex::new(Chopstick::new(2)));
    let cs3 = Arc::new(Mutex::new(Chopstick::new(3)));
    let cs4 = Arc::new(Mutex::new(Chopstick::new(4)));
    let cs5 = Arc::new(Mutex::new(Chopstick::new(5)));

    let mut handles = vec![];
    handles.push(deploy(1, cs1.clone(), cs2.clone()));
    handles.push(deploy(2, cs2.clone(), cs3.clone()));
    handles.push(deploy(3, cs3.clone(), cs4.clone()));
    handles.push(deploy(4, cs4.clone(), cs5.clone()));
    handles.push(deploy(5, cs5.clone(), cs1.clone()));

    for handle in handles {
        let _ = handle.join();
    }
}
