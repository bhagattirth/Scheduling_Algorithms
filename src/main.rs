mod pair;
mod d_wrap;
mod Process;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::BinaryHeap;


fn main() {
    // Process the text file and creates a priority quque of job process
    let workload:BinaryHeap<Process::Process> = read_and_load("Workload/workload_03.txt".to_string());

    // Passes a copy of the work load into each of the scheduling functions
    print_metrics(fifo(workload.clone()), "FIFO:".to_string());
    print_metrics(sjf(workload.clone()), "SHORTEST JOB FIRST:".to_string());
    print_metrics(stcf(workload.clone()), "SHORTEST JOB TO COMPLETION FIRST".to_string());
    print_metrics(rr(workload.clone()), "ROUND ROBIN".to_string());
    
}

//Process text file and returns a priority queue of jobs that need to be completed
fn read_and_load(filename: String) -> BinaryHeap<Process::Process> {
    //Creates a new priority queue for the job - ordered by arrival time
    let mut task:BinaryHeap<Process::Process> = BinaryHeap::new();
    //Opens the file and gets the content
    let file:File = File::open(filename).unwrap(); 
    let lines:io::Lines<io::BufReader<File>> =  io::BufReader::new(file).lines();

    //Extract each line one by one
    for line in lines {
        //The current line
        let curr:String = line.unwrap();
        // Splits the line into two - arrival time and duration
        let mut split = curr.split_whitespace();
        if let (Some(first), Some(second)) = (split.next(), split.next()) {
            let arrival: i32 = first.parse().unwrap();
            let duration: i32 = second.parse().unwrap();

            //Create a new process entry as defined in Process.rs and add it to the priority queue
            let temp = Process::Process::new(arrival, -1, duration, -1);
            task.push(temp);
        }
    }
    task
}


//Runs the Fifo scheduling algorithm returns a list of process with first_run and completion filled
fn fifo(tasks:BinaryHeap<Process::Process>) -> Vec<Process::Process>{

    //The list of jobs needed to be completed
    let mut work_load = tasks;
    //List of completed jobs
    let mut completed:Vec<Process::Process> = Vec::new();
    //Current Time
    let mut time = 0;

    //Runs until all jobs are completed
    while !work_load.is_empty(){
        //Gets the first arrived job
        let mut curr = work_load.pop().unwrap();
        
        // Completes the first_run field
        curr.first_run = time;

        //Increments time by the job duration
        time += curr.duration;
        curr.completion = time;

        // Adds the job to the completed list once its finished
        completed.push(curr);
    }
    completed
}


//Runs the sjf scheduling algorithm returns a list of process with first_run and completion filled
fn sjf(tasks:BinaryHeap<Process::Process>) -> Vec<Process::Process>{
    
    //The list of jobs needed to be completed
    let mut work_load:BinaryHeap<Process::Process> = tasks;
    // Jobs that currently being worked on - sorted by duration of job
    let mut work_list:BinaryHeap<d_wrap::Dur_Wrap> = BinaryHeap::new();
    //List of completed jobs
    let mut completed:Vec<Process::Process> = Vec::new();
    // Current time
    let mut time = 0;
    
    // Runs until all jobs arr completed
    while !work_load.is_empty() {

        // Adds jobs to work_list if new jobs have arrived at the moment
        while !work_load.is_empty() && work_load.peek().unwrap().arrival <= time{
          let new_load = d_wrap::Dur_Wrap::new(work_load.pop().unwrap());
          work_list.push(new_load);
        }
    
        // Works on the jobs in work_list before adding new jobs into the pq
        while !work_list.is_empty(){

          //The job with the shortest duration that has already arrived
          let mut curr = work_list.pop().unwrap().process;

          // Completes the first_run field
          curr.first_run = time;

          //Increments time by the job duration
          time += curr.duration;
          curr.completion = time;

          // Adds the job to the completed list once its finished
          completed.push(curr);
        }
   
    }
    completed
}

//Runs the stcf scheduling algorithm returns a list of process with first_run and completion filled
fn stcf(tasks:BinaryHeap<Process::Process>) -> Vec<Process::Process>{
    // The list of jobs needed to be completed
    let mut work_load = tasks;
    // Jobs that currently being worked on - sorted by remaining time until job completion
    let mut work_list:BinaryHeap<pair::Pair> = BinaryHeap::new();
    // List of completed jobs
    let mut completed:Vec<Process::Process> = Vec::new();
    // Current time
    let mut time:i32 = 0;
    // Keeps track of when the next job is arriving
    let mut next_job_in:i32 = 0;



  // Runs until all jobs are completed
  while !work_load.is_empty(){  

    // Adds jobs to work_list if new jobs have arrived at the moment
    while !work_load.is_empty() && work_load.peek().unwrap().arrival <= time{
        let new_load = work_load.pop().unwrap();
        // work_list takes a pair which is defined as (time until job completion, process)
        let pair = pair::Pair::new(new_load.duration, new_load); 
        work_list.push(pair);
    }
    
    while !work_list.is_empty() {

      //Takses the job with the shortest time til completion that has arrived
      let pair = work_list.pop().unwrap();
      // The job that is being worked on
      let mut job = pair.process;
      // Time remaining until the current job is completed
      let mut remaining_time = pair.remaining;

      // if the job hasn't been ran before, update first_run of the job
      if job.first_run == -1 {
         job.first_run = time;
      }

      //Checks if there are more jobs arriving in the future
      if !work_load.is_empty() {
        //Calculates when the next job arrives
        next_job_in = work_load.peek().unwrap().arrival - time;

        // Need to take the min of both if a new job arrives before the current job is done
        let max_time = cmp::min(remaining_time, next_job_in);

        //Update how much time has passed since working on this current job
        remaining_time -= max_time;
        time += max_time;

        // Should be zero if remainng_time > next_job_in, meaning a new job has arrived
        next_job_in -= max_time;
      }else{
        // If there is no incoming jobs, complete the remaining jobs using sjf
        next_job_in = -1;
        time += remaining_time;
        remaining_time = 0;
      } 
      
      // if the current job isn't complete add it back into worklist else update the job's completion time and add it to the list
      if remaining_time != 0{
        work_list.push(pair::Pair::new(remaining_time, job));
      }else{
        job.completion = time;
        completed.push(job);
      }
      
      //We must add the new jobs before working on work list again
      if next_job_in == 0 {
        break;
      } 
    }
  }
  completed
}

//Runs the rr scheduling algorithm returns a list of process with first_run and completion filled
fn rr(tasks:BinaryHeap<Process::Process>) -> Vec<Process::Process> {

    // The list of jobs needed to be completed
    let mut work_load: BinaryHeap<Process::Process> = tasks;
    // Jobs that currently being worked on - sorted with fifo
    let mut work_list: Vec<pair::Pair> = Vec::new();
    // List of completed jobs
    let mut completed: Vec<Process::Process> = Vec::new();
    // current time
    let mut time = 0;
    // Keeps track when a new job is arriving
    let mut next_job_in = 0;
 

    // Runs until all jobs are completed
    while !work_load.is_empty() {
      // Adds jobs to work_list if new jobs have arrived at the moment
      while !work_load.is_empty() && work_load.peek().unwrap().arrival <= time {
        let new_load = work_load.pop().unwrap();
        work_list.push(pair::Pair::new(0, new_load));
      }
      
      while !work_list.is_empty() {
        // Takes the job first from the queue of those that have arrived
        let pair = work_list.remove(0);

        // The job that is being worked and number of rounds the process has completed
        let mut rounds = pair.remaining;
        let mut curr = pair.process;

        //If this is the current job's first time running, update the first_run field
        if curr.first_run == -1 {
           curr.first_run = time;
        }   
  
        // Apply RR on the job for one second
        rounds+=1;
        time+=1;

        // Defaults to negative -1
        next_job_in = -1;
  
        //Checks if new jobs will arrive in the future
        if !work_load.is_empty(){ 
          //Calculates when the new jobs will arrive
          next_job_in = work_load.peek().unwrap().arrival - time;
        }
  
        // If the job isn't completed, add it to the back of the queue
        if rounds != curr.duration{
          work_list.push(pair::Pair::new(rounds, curr));
        }else{
          // Else update the jobs completion field and add it to the list
          curr.completion = time;
          completed.push(curr);
        }
        // We must add the new jobs before working again
        if next_job_in == 0 { 
          break;
        }
      }
    }
    completed
  }

// Calculates Average Response
fn calculate_response(processes: Vec<Process::Process>) -> f64{
  let size = processes.len() as i32;
  let mut average = 0;
  for process in processes{
    average += process.first_run - process.arrival;
  }
  return average as f64 /size as f64;
}
  
// Calculates Average turnaround
fn calculate_turnaround(processes: Vec<Process::Process>) -> f64{
  let size = processes.len() as i32;
  let mut average = 0;
  for process in processes{
    average += process.completion - process.arrival;
  }
  return average as f64 /size as f64;
}

// Prints All Relevant Metrics
fn print_metrics(processes: Vec<Process::Process>, header:String){
  let a1 = processes.clone();
  let a2 = processes.clone();
  println!("\n{}", header);
  for process in processes{
      println!("Arrival: {}, Duration: {}, First_run: {}, Completed: {}", process.arrival, process.duration,  process.first_run, process.completion);
    }
    println!("Average Turnaround Time: {}, Average Response Time: {}", calculate_turnaround(a1), calculate_response(a2));
}

