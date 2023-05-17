use crate::Process::Process;
use std::cmp::Ordering;

#[derive(Eq, Clone)]

pub struct Pair {
    pub remaining: i32,
    pub process: Process,   
}

impl Pair {
    pub fn new(remaining: i32, process: Process) -> Pair {
        Pair{remaining, process}
    }
}

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