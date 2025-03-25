# effectiveTransferTestTask
## Description
This project contains a Rust client application to download binary 
data from a glitchy HTTP server. The server may send data partially, 
so the client uses the HTTP "Range" header to complete the download.

## Features
- Support for both synchronous and asynchronous data downloading.
- Data integrity verification using SHA-256 hash.
- Option to choose between synchronous and asynchronous modes.

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
