use crate::Process::Process;

#[derive(Clone)]

pub struct UPair {
    pub remaining: i32,
    pub process: Process,   
}

impl UPair {
    pub fn new(remaining: i32, process: Process) -> UPair {
        UPair{remaining, process}
    }
}