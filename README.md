# Scheduling Algorithms in Rust

---
**The goal of this project was to convert the Homework 3's C++ homework about scheduling algorithms into a Rust Project.**<br>

---
## Implementation Details:

### Miscalaneous: 

**Structs -** <br>
In this project, I have defined three structs that I find useful: Process, Dur_wrap, and Pair.

The Process struct contains essential information about each job, such as its arrival time, duration, the time it first ran, and its completion time. I have kept this struct unchanged from the homework, as it is the most intuitive way to store the information. In Rust, there are no comparators, so each struct must come with a predefined comparison mechanism, which is useful for a priority queue. In the case of Process, we sort by the smallest arrival time first.

Dur_wrap is simply a wrapper for the Process struct. The Dur_wrap struct contains only one field, which stores a Process. I needed Dur_wrap because Rust does not have comparators, so each struct must have a predefined ordering system. Since there is no direct way to change the ordering of Process based on the smallest arrival time, I wrap it with Dur_wrap, which introduces a new ordering system that can be used by a priority queue. In this case, Dur_wrap allows us to sort jobs by their duration.

The Pair struct is another wrapper. Pair stores a Process and the remaining time until its completion. Similar to the reason for using Dur_wrap, I created the Pair struct to enable sorting of jobs by the remaining time until completion and to keep track of the time until job completion. <br><br>


**Inputs -** <br>
The Project takes in a file that contains jobs. At each line there are two numbers, the job arrival time and the duration of the job. <br><br>

**Loading Function - read_and_load() -** <br>
The job file is opened and the contents are extracted. Each job is placed inside a Process struct, which includes essential information such as the job's arrival time, duration, the time it first ran, and its completion time. Initially, the arrival time and duration fields are filled, while the first run and completion times are set to -1. I chose to set them as -1 because it allows for easy checking to determine if this is the job's first time running, which is particularly useful for SJCF and RR algorithms, when we have put jobs back into the queue again (look a the implementation for more detail).

Following that, the jobs are inserted into a binary heap (or priority queue) that is sorted based on the job's arrival time. <br><br>

### Scheduling Algorithms:

**FIFO -** <br>
The FIFO algorithm operates based on the arrival time of jobs and does not take into consideration their duration. Within the algorithm, we iterate through all the jobs in the binary heap and update their values. For each job, it starts by updating the job's "first_run" field and then increments the current time variable by the duration of the job, simulating the work being done on that job. Subsequently, it updates the job's "completion" field. When the job is completed, we add it to the list of completed jobs and return the list. <br><br>

**SJF -** <br>
The SJF algorithm operates by prioritizing jobs based on their shortest duration, focusing only on those that have already arrived, rather than those that will arrive in the future. The algorithm consists of three while loops. The outer loop ensures that all jobs are processed, while the inner loops ensure that we work on jobs with the shortest duration among those that have arrived, and perform operations on them.

Inside the first loop, we immediately enter the second loop. The second loop, as mentioned earlier, retrieves all the arrived jobs and sorts them based on their duration. To sort by duration, we need to use Dur_Wrap on the Processes (jobs). Moving on to the third loop, we essentially carry out the same operations as the FIFO algorithm. We iterate through each job, updating the first run field, incrementing the time, and populating the job's completion field. We repeat these steps until all the jobs are completed. <br><br>

**STCF -** <br>
The STCF algorithm is similar to the SJF algorithm but considers future jobs that may come. Similarly, the algorithm also has three loops that perform similar tasks. However, there are two main differences. The first difference is the use of the Pair wrapper struct instead of Dur_Wrap. It is used to keep track of how much time is remaining until the job is completed and sort it by the shortest remaining time on the priority queue (and act like a "save state" if we need to add this job back into the queue). The second main difference lies in the implementation of the third loop.

In this section, we operate on the job with the shortest time until completion among those that have arrived. If the job has never been run before, we update the field with the current time variable. Next, we need to check if there will be future jobs by examining the workload priority queue. If there are any, we keep track of how much time is left until the next job appears using the variable 'next_job_in', which is obtained by calculating the difference between the arrival time of the future job and the current time. Until a new job arrives or there is no upcoming job, the code behaves similarly to STCF.

We iterate through each job, updating the 'first_run' field if it is the first time the job is being executed. We then simulate the CPU working on it by advancing the time and moving it to the completed list if the job is finished. However, if a new job has arrived or 'next_job_in' equals zero, we stop working on the current job and push it back into the secondary priority queue with its new remaining time. We then break out of the third loop to add the new jobs to the current job list. From there, we select the new job with the smallest remaining time and repeat the process until all the jobs are completed. <br><br>

**RR -** <br>
The Round-Robin (RR) algorithm operates on each job for a short period of time before switching to another job and repeating the process until all jobs are completed. The implementation of RR is quite similar to STCF with a few changes.

The first difference is with work_list or the jobs that have arrived. In RR, the work_list is organized as a queue, without any sorting based on metrics, in order to provide each process with a fair opportunity of cpu time.

In the third loop, instead of attempting to complete a job in one go, we allocate only 1 second of processing time to each job before moving on to the next one. This approach aims to ensure fairness among the jobs. Once the CPU has spent an equal amount of time on a job as the job's specified duration, we update the job's completion field and add it to the completed list.

Other than that, RR runs similarly to the STCF algorithm. <br><br>

### Metrics:

**Print_Metrics -** <br>
The function takes a list of completed jobs and prints each job's field (arrival, duration, first run, and completion) as well as the average turnarround and response time. <br><br>

**calculate_turnarround -** <br>
The function takes a completed list and computes the average turnarround by averaging each jobs turnarround (completion - arrival). <br><br>

**calculate_response -** <br>
The function takes a completed list and computes the average response time by averaging each jobs response time (first run - arrival).



