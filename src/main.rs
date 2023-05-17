
mod Process;
mod d_wrap;
mod u_pair;
mod pair;
use std::io::{self, BufRead};
use std::fs::File;
use std::collections::BinaryHeap;
use std::collections::LinkedList;
use std::cmp;

// #[derive(debug)]
fn main() {
    let workload = read_and_load("Workload/workload_03.txt".to_string());
    fifo(workload.clone());
    sjf(workload.clone());
    stcf(workload.clone());
    rr(workload.clone());

}


fn read_and_load(filename: String) -> BinaryHeap<Process::Process> {
    let mut task:BinaryHeap<Process::Process> = BinaryHeap::new();
    let file = File::open(filename).unwrap(); 
    let lines =  io::BufReader::new(file).lines();

    for line in lines {
        let curr = line.unwrap();
        let mut split = curr.split_whitespace();
        if let (Some(first), Some(second)) = (split.next(), split.next()) {
            let arrival: i32 = first.parse().unwrap();
            let duration: i32 = second.parse().unwrap();
            let temp = Process::Process::new(arrival, -1, duration, -1);
            task.push(temp);
        }
    }
 
    // while !task.is_empty() {
    //     let text = task.pop().unwrap();
    //     println!("arrival: {}, first_run: {}, duration: {}, completion :{} ", text.arrival, text.first_run, text.duration, text.completion);
    // }
    task
}

fn fifo(tasks:BinaryHeap<Process::Process>) -> Vec<Process::Process>{
    let mut work_load = tasks;
    let mut completed:Vec<Process::Process> = Vec::new();
    let mut time = 0;

    while !work_load.is_empty(){
        let mut curr = work_load.pop().unwrap();
        curr.first_run = time;
        time += curr.duration;
        curr.completion = time;
        completed.push(curr);
    }
    completed

    // for text in completed {
    //     println!("arrival: {}, first_run: {}, duration: {}, completion :{} ", text.arrival, text.first_run, text.duration, text.completion);
    // }
}

fn sjf(tasks:BinaryHeap<Process::Process>) -> Vec<Process::Process>{
    let mut work_load = tasks;
    let mut work_list:BinaryHeap<d_wrap::Dur_Wrap> = BinaryHeap::new();
    let mut completed:Vec<Process::Process> = Vec::new();
    let mut time = 0;
    

    while !work_load.is_empty() {
        while !work_load.is_empty() && work_load.peek().unwrap().arrival <= time{
          let new_load = d_wrap::Dur_Wrap::new(work_load.pop().unwrap());
          work_list.push(new_load);
        }
    
        while !work_list.is_empty(){
          let mut curr = work_list.pop().unwrap().process;
          curr.first_run = time;
          time += curr.duration;
          curr.completion = time;
          completed.push(curr);
        }
    //   complete;
    }
    // for text in completed {
    //     println!("arrival: {}, first_run: {}, duration: {}, completion :{} ", text.arrival, text.first_run, text.duration, text.completion);
    // }
    completed
}

fn stcf(tasks:BinaryHeap<Process::Process>) -> Vec<Process::Process>{
    let mut work_load = tasks;
    let mut work_list:BinaryHeap<pair::Pair> = BinaryHeap::new();
    let mut completed:Vec<Process::Process> = Vec::new();
    let mut time = 0;
    let mut next_job_in = 0;

  while !work_load.is_empty(){  
    while !work_load.is_empty() && work_load.peek().unwrap().arrival <= time{
        let new_load = work_load.pop().unwrap();
        let pair = pair::Pair::new(new_load.duration, new_load); 
        work_list.push(pair);
    }
    
    while !work_list.is_empty() {
      let pair = work_list.pop().unwrap();
      let mut job = pair.process;
      let mut remaining_time = pair.remaining;

      if job.first_run == -1 {
         job.first_run = time;
      }

      if !work_load.is_empty() {
        next_job_in = work_load.peek().unwrap().arrival - time;
        let max_time = cmp::min(remaining_time, next_job_in);
        remaining_time -= max_time;
        time += max_time;
        next_job_in -= max_time;
      }else{
        next_job_in = -1;
        time += remaining_time;
        remaining_time = 0;
      } 

      if remaining_time != 0{
        work_list.push(pair::Pair::new(remaining_time, job));
      }else{
        job.completion = time;
        completed.push(job);
      }
      
      if next_job_in == 0 {
        break;
      } 

    }
  }
//   for text in completed {
//     println!("arrival: {}, first_run: {}, duration: {}, completion :{} ", text.arrival, text.first_run, text.duration, text.completion);
// }
   completed
}


fn rr(tasks:BinaryHeap<Process::Process>) {
    let mut work_load: BinaryHeap<Process::Process> = tasks;
    let mut work_list: Vec<pair::Pair> = Vec::new();
    let mut completed: Vec<Process::Process> = Vec::new();
    let mut time = 0;
    let mut next_job_in = 0;
 
    // println!("{}", work_load.len());
    while !work_load.is_empty() {
      while !work_load.is_empty() && work_load.peek().unwrap().arrival <= time {
        let new_load = work_load.pop().unwrap();
        println!("{}", work_load.len());
        work_list.push(pair::Pair::new(0, new_load));
      }
      
      while !work_list.is_empty() {
        let pair = work_list.remove(0);
        let mut rounds = pair.remaining;
        let mut curr = pair.process;
        println!("Popped: {}, rounds: {}", curr.duration, rounds);

        if curr.first_run == -1 {
           curr.first_run = time;
        }   
  
        rounds+=1;
        time+=1;
        next_job_in = -1;
  
        if !work_load.is_empty(){ 
          next_job_in = work_load.peek().unwrap().arrival - time;
        }
  
        if rounds != curr.duration{
          work_list.push(pair::Pair::new(rounds, curr));
          println!("First {}", work_list[0].process.duration);
        }else{
          curr.completion = time;
          completed.push(curr);
        }
        if next_job_in == 0 { 
          break;
        }
      }
    }
      for text in completed {
        println!("arrival: {}, first_run: {}, duration: {}, completion :{} ", text.arrival, text.first_run, text.duration, text.completion);
    }
    // return completed;
  }


