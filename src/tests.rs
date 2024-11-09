use std::time::SystemTime;
use crate::*;

pub struct TimingDebugger {
    _start_time: SystemTime
}
impl TimingDebugger {
    pub fn new() -> TimingDebugger {
        TimingDebugger {_start_time: SystemTime::now()}
    }

    pub fn breakpoint() {
        println!("[?] Press 'ENTER' when you're ready to continue.");
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    pub fn checkpoint(&mut self, identifier: &str) {
        let duration = SystemTime::now().duration_since(self._start_time).expect("Time went backwards");
        self._start_time = SystemTime::now();

        println!("[{}] Duration since last checkpoint: {}", identifier.to_ascii_uppercase(), duration.as_millis());
        TimingDebugger::breakpoint();
    }
}