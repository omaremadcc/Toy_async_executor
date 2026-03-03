use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct Executor {
    os_tx: Sender<u64>,
    notifications_rx: Receiver<u64>,
}

impl Executor {
    pub fn new() -> Self {
        let (os_tx, os_rx): (Sender<u64>, Receiver<u64>) = std::sync::mpsc::channel();
        let (notifications_tx, notifications_rx): (Sender<u64>, Receiver<u64>) =
            std::sync::mpsc::channel();
        let _ = thread::spawn(move || {
            while let Ok(index) = os_rx.recv() {
                thread::sleep(std::time::Duration::from_secs(3));
                println!("The Court Is emptied");
                let _ = notifications_tx.send(index);
            }
        });
        return Self {
            os_tx,
            notifications_rx,
        };
    }

    pub fn exeutor(self, mut vec: Vec<Job>) {
        loop {
            if vec.iter().find(|f| f.enabled).is_none() {
                match self.notifications_rx.recv() {
                    Ok(index) => {
                        if let Some(job) = vec.iter_mut().find(|j| j.id == index) {
                            job.enabled = true;
                        }
                    }
                    Err(_) => continue,
                }
                loop {
                    match self.notifications_rx.try_recv() {
                        Ok(index) => {
                            let _ = vec.iter_mut().for_each(|f| {
                                if f.id == index {
                                    f.enabled = true
                                }
                            });
                        }
                        Err(_) => break,
                    }
                }
            }
            let mut i = 0;
            while i < vec.len() {
                if !vec[i].enabled {
                    i += 1;
                    continue;
                }
                let result = (vec[i].func)(self.os_tx.clone());
                match result {
                    MineFuture::Pending => {
                        vec[i].enabled = false;
                        i += 1;
                    }
                    MineFuture::Success(value) => {
                        vec.remove(i);
                        println!("Finished task and it returned {}", value);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum MineFuture {
    Pending,
    Success(String),
}
pub type JobFn = fn(tx: Sender<u64>) -> MineFuture;

pub struct Job {
    pub id: u64,
    pub enabled: bool,
    pub func: JobFn,
}
impl Job {
    pub fn new(id: u64, f: JobFn) -> Self {
        Self {
            id,
            enabled: true,
            func: f,
        }
    }
}
