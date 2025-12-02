use std::time::SystemTime;

pub struct RaiiTime {
    now: std::time::SystemTime,
}

impl RaiiTime {
    pub fn new() -> RaiiTime {
        RaiiTime {
            now: SystemTime::now(),
        }
    }
}

impl Drop for RaiiTime {
    fn drop(&mut self) {
        println!(
            "Executed func in {} microseconds",
            self.now.elapsed().unwrap().as_micros()
        );
    }
}
