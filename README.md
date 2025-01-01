# Unsafe Rust Pointer Manipulation Leading to Data Corruption
This repository demonstrates a potential issue with directly manipulating memory using unsafe Rust.  Modifying a vector through a raw pointer can lead to unexpected behavior or data corruption if not handled carefully.

The `bug.rs` file contains code that exemplifies this problem.  The `bugSolution.rs` file presents a safer alternative.

This example highlights the importance of understanding the implications of unsafe Rust and the need for meticulous attention to detail when working with raw pointers.