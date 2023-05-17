# Scheduling Algorithms in Rust

---
**The Goal of this project was to convert the Homework 3's C++ homework about scheduling algorithms into a Rust Project.**

---
## Implementation Details:

### Miscalaneous:

**Structs:**
In this project, I defined 3 structs that I thought were useful for this project: Process, Dur_wrap, and Pair.

The process struct contains essential information about each job such as it's arrival time, duration, when it first ran, and when it was completed. I kept this as is from the homework since it is most intutive way of storing the information. In Rust, there are no comparators, so each struct must come with a predefine comparing mechanism (useful for a priority queue). For Process, we sort by the smallest arrival time first.

Dur_wrap is simply a wrapper for Process struct. Dur_Wrap struct contains only one field which stores a wrapper. I needed the Dur_Wrap because there is no comparators in Rust so each Struct has a predefined ordering system. Since there is no way to change Process's smallest arrival time ordering directly, we wrap it with Dur_Wrap with a new ordering system that a priority queue can use. In this case Dur_Wrap allows us to sort by a job's duration.

The Pair Struct another wrapper. Pair stores a Process and the remaining time until it completed. 

### Inputs - 
The Project takes in a file that contains jobs. At each line there are two numbers, the job arrival time and the duration of the job. 

### Loading Function - read_and_load() -
The job file is opened and the file is exacted. Each of the jobs are put inside a Process struct which contains essential information such as the job's arrival time, duration, when it first ran, and when it was completed. Intially, the arrival time and duration are filled. And the first ran and completed time are set to -1. I choose to set them to -1 because it allows me to easaily check if this is the job's first time running which is very helpful for the SJCF and RR (look at the functions for more details). 

After that, the jobs are put inside a binary heap (or priority queue) which is sorted by job's arrival time.





