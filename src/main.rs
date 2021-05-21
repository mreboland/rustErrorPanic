fn main() {
    println!("Hello, world!");


    // Error Handling

    // Ordinary errors are handled using Results. These are typically caused by things outside the program, like erroneous input, a network outage, or a permissions problem.



    // Panic

    // A program panics when it encounter something so messed up that there must be a bug in the program itself. Something like:
    // 1. Out-of-bounds array access
    // 2. Integer division by zero
    // 3. Calling .unwrap() on an Option that happens to be None
    // 4. Assertion failure

    // There's also the macro panic!(), for cases where our own code discovers that it has gone wrong, and we therefore need to trigger a panic directly. panic!() accepts optional println!()-style arguments, for building an error message.

    // What all the above errors have in common is that they are the programmer's fault. When errors that shouldn't happen, do happen, what then? Rust gives us a choice. Rust can either unwind the stack when a panic happens, or abort the process. Unwinding is the default.



    // Unwinding

    // When pirates divvy up the booty from a raid, the captain gets half of the loot. Ordinary crew members earn equal shares of the other half. If the division doesn't come out even, the result is rounded down, the remainder going to the ship's parrot.
    fn pirate_share(total: u64, crew_size: usize) -> u64 {
        let half = total / 2;
        half / crew_size as u64
    }

    // This may work fine for centuries until one day it transpires that the captain is the sole surviver of a raid. If we pass a crew_size of zero to this function, it will divide by zero. In C++, this would be undefined behaviour. In Rust, it triggers a panic, which typically proceeds as follows:
    // 1. An error message is printed to the terminal:
    // thread 'main' panicked at 'attempt to divide by zero', pirates.rs:3780
    // note: Run with `RUST_BACKTRACE=1` for a backtrace.
    // If we set the backtrace environment var, as the messages suggests, Rust will also dump the stack at this point.
    // 2. The stack is unwound. This is a lot like C++ exception handling.
    // Any temp values, local vars, or arguments that the current function was using are dropped, in the reverse of the order they were created. Dropping a value simply means cleaning up after it. Any Strings or Vecs the program was using are freed, any open Files are closed, and so on.User-defined drop methods are called too. See "Drop" in chapter 13. In the particular case of pirate_share(), there's nothing to clean up.
    // Once the current function call is cleaned up, we move on to its caller, dropping its variables and arguments the same way. Then that function's caller, and so on up the stack.
    // 3. Finally, the thread exits. If the panicking thread was the main thread, then the whole process exits (with a nonzero exit code).

    // A panic is not a crash. It's not undefined behaviour. It's more like a RuntimeException in Java, or a logic error in C++. The behaviour is well-defined, it just shouldn't be happening.

    // Panic is safe. It doesn't violate any of Rust's safety rules. Even if we manage to panic in the middle of a standard library method, it will never leave a dangling pointer or a half-initialized value in memory. The idea is that Rust catches the invalid array access, or whatever it is, before anything bad happens. It would be unsafe to proceed, so Rust unwinds the stack. But the rest of the process can continue running.

    // Panic is per thread. One thread can be panicking while other threads are going on about their normal business. In chapt 19, we'll show how a parent thread can find out when a child thread panics and handle the error gracefully.

    // There is also a way to catch stack unwinding, allowing the thread to survive and continue running. The standard lib function std::panic::catch_unwind() does this. This is the mechanism used by Rust's test harness to recover when an assertion fails in a test. It could be necessary when calling Rust code in C or C++ because unwinding is undefined behaviour in those languages.

    // We can use threads and catch_unwind() to handle panic, making our program more robust. One important caveat is that these tools only catch panics that unwind the stack. Not every panic proceeds this way.

    

    // Aborting

    // Stack unwinding is the default panic behaviour, but there are two circumstances in which Rust does not try to unwind the stack.

    // If a .drop() method triggers a second panic while Rust is still trying to clean up after the first, this is considered fatal. Rust stops unwinding and aborts the whole process.

    // Also, Rust's panic behaviour is customizable. If we compile with -C panic=abort, the first panic in our program immediately aborts the process. With this option, Rust doesn't need to know how to unwind the stack, so this can reduce the size of our compiled code.

}
