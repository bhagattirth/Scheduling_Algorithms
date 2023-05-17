use crate::Process::Process;
use std::cmp::Ordering;

#[derive(Eq, Clone)]

// This is essentially a pair class where remaining is the remaining time until job completion and processs is the job
pub struct Pair {
    pub remaining: i32,
    pub process: Process,   
}

impl Pair {
    pub fn new(remaining: i32, process: Process) -> Pair {
        Pair{remaining, process}
    }
}

// The code is below defines the ordering in pair class. We order by the shortest remaining time
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.remaining.cmp(&self.remaining)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.remaining == other.remaining
    }
}