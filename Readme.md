# effectiveTransferTestTask
## Description
This project contains a Rust client application to download binary 
data from a glitchy HTTP server. The server may send data partially, 
so the client uses the HTTP "Range" header to complete the download.

## Features
- Support for both synchronous and asynchronous data downloading.
- Data integrity verification using SHA-256 hash.

## Idea
The client application is designed with two solutions in mind:
1. **Synchronous Download**: This approach handles the download process 
sequentially, ensuring each part of the data is received before 
moving on to the next. It is straightforward and easy to implement 
but may not be the most efficient under high load conditions.

2. **Asynchronous Download**: This approach allows multiple parts of the 
data to be downloaded concurrently, potentially speeding up the process, 
especially when dealing with large files or slow networks. It leverages 
Rust's async capabilities to manage multiple download tasks efficiently.

After writing both solutions, I will test them with hyperfine and choose more optimal one, 
leaving both in the final version of the application for anyone to reconsider usage of less
optimal one.

## Implementation notes
### Some details and thoughts
- The application uses the `reqwest` crate for HTTP requests and 
  `tokio` for asynchronous operations.
- The SHA-256 hash is computed using the `sha2` crate.
- The application is designed to be run in synchronous or asynchronous mode 
  based on features enabled
- All libraries that I tried hate when body length is less than content-length
  header, so I had to use some tricks to make it work.
- I couldn't made a launch in async mode that would be faster than sync one, 
  so I left it as an option for anyone to reconsider usage of less optimal one.

### Why synchronous mode faster?
It is faster because of the server implementation. When server is 'sleeping' to make my
 life harder, all requests are being denied and I have to resend it again. Also server is
 synchronous, so it is not able to handle multiple requests at once, so, it's kinda intuitive,
 but I didn't expect it to be that much faster (also I believed to the hype about async evaluations,
 and, knowing that it's superb in most of real applications, 
 it wasn't there because of specifics)

### Why I even implemented async mode?
I thought that it's somewhat cool to have both options, 
 because even if async mode is slower, it may become a better
 option in the future, when server will be able to handle multiple 
 requests at once. Also, firstly I thought that it would be faster,
 so it was a good idea to implement it.

### Why I implemented sync mode?
Because it just transfers less data and it's cool to have it
 as an option. Also, I thought that it would be slower, 
 but something inside me told me that experiment is worth it (and it is!).

## Usage
### Prerequisites
- Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).
- `cargo` package manager (comes with Rust installation).
- `hyperfine` for benchmarking (optional, but recommended, I didn't use it, because difference
  was obvious, but it would be nice to have it in mind in case, 
  so I added it to the readme)
- `git` for cloning the repository.

### Cloning the repository
```bash
git clone https://github.com/miko089/effectiveTransferTestTask
```
### Building the project
```bash
cd effectiveTransferTestTask
cargo build --release --features sync
```
(or `async` if you want to build async version)
### Running the project
```bash
./target/release/effectiveTransferTestTask
```

## How fast it is?
Sync version works faster than async one, taking from 0.7s to 4s to execute,
while async one takes from 3s to 8s to execute.

## Contact
If you have any questions or suggestions, feel free to reach out to me:

telegram: [@miko089](http://t.me/miko089)

email: ghastmisha089@gmail.com



