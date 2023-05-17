use std::cmp::Ordering;
#[derive( Eq, Clone, Copy)]
//This Represents a job that might run in the cpu
pub struct Process {
   // job arrival time
   pub arrival: i32,
   // When it first ran
   pub first_run: i32,
   // Duration of the job
   pub duration: i32,
   // Time of completion
   pub completion: i32,
}

impl Process {
    pub fn new(arrival: i32, first_run: i32, duration: i32, completion: i32) -> Process {
        Process{arrival, first_run, duration, completion }
    }
}

// Defines the ordering of Process in a priority queue - order by earliest arrival time
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



