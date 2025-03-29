Implement a job queue system in Rust with the following requirements:

1. Create a job queue that can store and process different types of jobs
2. Implement two job types:
    - EmailJob: Simulates sending an email (sleep for 2 seconds)
    - DataProcessingJob: Simulates processing data (sleep for 3 seconds)
3. Each job takes in a shared variable at the time of processing. The shared variable tracks total number of processed jobs and must be incremented at the end of the job
4. The queue should have a configurable maximum capacity
5. Implement a worker pool that processes jobs concurrently with a configurable number of workers
6. The system should gracefully handle queue full/empty conditions
7. Provide methods to submit jobs and shut down the queue

### Pointers

1. Feel free to use the internet for docs or references. Use of AI tools is not allowed.