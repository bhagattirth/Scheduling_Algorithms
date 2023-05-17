use crate::Process::Process;
use std::cmp::Ordering;
#[derive(Eq, Clone)]

pub struct Dur_Wrap {
    pub process: Process,   
}

impl Dur_Wrap {
    pub fn new(process:Process) -> Dur_Wrap {
        Dur_Wrap{process}
    }
}

impl Ord for Dur_Wrap {
    fn cmp(&self, other: &Self) -> Ordering {
        other.process.duration.cmp(&self.process.duration)
    }
}

impl PartialOrd for Dur_Wrap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Dur_Wrap {
    fn eq(&self, other: &Self) -> bool {
        self.process.duration == other.process.duration
    }
}