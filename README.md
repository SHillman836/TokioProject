# Tokio Project README

### Purpose and Functionality:
This is a command line project exploring async in Rust. 

An input text file containing a series of bytes corresponding to a vector of the numbers 1-20 is automatically generated when you run the program. 

For the purposes of this project we're only using the numbers 1-20, however the way we deal with this data makes sense if we were dealing with large amounts of data. That's really the point of this project, to learn to handle large amounts of data using async Rust.

Broadly this is what the program does:
1. Reads the bytes in chunks of 4 (which reads 1 integer at a time).
2. Give each integer an index.
3. Subject each integer to a computation which we specify when running the program from the command line.
4. Write the result of the computation with it's corresponding index to an output file.

We use an actor model to facilitate async. We have one actor reading the input text file and packages these chunk of bytes into a message and sends them to the router. The router then sends this message to one of two transformation actor workers, alternating each time. 

The transformation actor is responsible for performing the computation. This is where async really comes into play as if a particular computation was taking a long time we'd have another worker free to keep computing, allowing us to continue to process new chunks of data. The router acts as a load balancer in this way. 

The transformation actor then sends the result of the computation along with its index to our file writer actor, which writes it to our output file.

A few extra things to note:
1. The reason we chunk the data 4 bytes at a time is that if we were dealing with a file containing large amounts of data we wouldn't be able to load it all in at once, we'd have to chunk it.
2. In this program we have 2 simple computations that the user can choose from when running the program from the command line - squaring the integer or square rooting it. For such simple computations async wouldn't really have an advantage. However, if we were performing more costly computations async would give us an advantage.
3. The reason we attach each integer to an index is to show how async executes these computation concurrently. That's the beauty of async, it allows multiple tasks to take place concurrently, meaning they finish at different times too. This is reflected in the order of the indexes when all integers are written to file.

### How to run the program
1. First please ensure you have rust installed on your computer. You can install it here `https://www.rust-lang.org/tools/install`.
2. Then clone the project into your local files.
3. Navigate to the root folder and run `cargo run <computation>`. You have 2 choices of computation - square and root. Square squares every integer in the input file before writing it, and root square roots them.





