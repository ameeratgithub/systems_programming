# Introduction

This project contains a lot of examples and some mini projects related to systems programming concepts. Examples are standalone and you should be able to run them without any problem. As all projects are in this single crate, you've to explicitly edit the `main.rs` to run the desired project.

## Projects
Following projects demonstrate the understanding of systems programming concepts in depth.

- **CPU Emulator:** This projects illustrates how data is responsible for everything in computers, even functions are data. CPU Emulator defines everything in bytes, introduces concept of memory, how data should be loaded into memory, and how registers are responsible to fetch data from memory and perform operations.

- **Heap Visualizer:** This project displays how many bytes of data being requested in current program and how much time does it take. Heap Visualizer does so by allocating a lot of objects on heap, and printing the time and data requested on console. If you want to tune your system for performance, these concepts matter.

- **Key Value Store:** This project is all about storing data on disk and making it durable. Indexing, checksum for data integrity and common operations to store and retrieve values are introduced.

## Examples
There are a lot of small examples illustrating different systems programming concepts. Each category/type is located in its separate directory. Each of the type is discussed below

- **Data:** This directory includes example related to data representation in computer. What are integers? How they are represented? Decimal Representation? Bit patterns of different types.

- **Memory:** Memory section is all about, well, memory, explaining stack and heap. When we talk about heap, then there are multiple concepts including pointers, references, raw pointers and smart pointers. Also you can see examples about scanning the program memory.

- **Files and Storage:** This section discusses the different data formats and how to write to disk. Different data formats have different trade-offs we need to consider while designing a database or similar system.