

pub struct Tracker {
    workers: Vec<Worker>,
    receiver: Arc<Mutex<mpsc::Receiver<Event>>>,
    delay_in_msec: u64,
    sample_count: u64,
    total_count: u64,
}

impl Tracker {
    pub fn new(config: &super::config::Config) -> Tracker {
    }
}
