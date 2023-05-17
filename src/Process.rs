use std::cmp::Ordering;
#[derive( Eq, Clone)]
pub struct Process {
   pub arrival: i32,
   pub first_run: i32,
   pub duration: i32,
   pub completion: i32,
}

impl Process {
    pub fn new(arrival: i32, first_run: i32, duration: i32, completion: i32) -> Process {
        Process{arrival, first_run, duration, completion }
    }
}

impl Ord for Process {
    fn cmp(&self, other: &Self) -> Ordering {
        other.arrival.cmp(&self.arrival)
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.arrival == other.arrival
    }
}



