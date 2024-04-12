fn main() {
    // Rust threading is portable, so this code should work across Mac, Linux, Windows

    // Here is a fully functional but pretty empty example
    // We bring the thread module into scope first then in our main function we call `thread spawn` thread spawn takes a closure with no arguments. This closure is executed as the main function of the thread. So anything that we would want to do in our thread, we would do here. 
    // One common practice is to simply call a function from the closure, so we don't have a big huge long in-line closure.

    use std::thread;

    let handle = thread::spawn(move || {
        // do stuff in a child thread
    });

    // do stuff simultaneously in the main thread

    // Spawn returns a joint handle, with that handle, we can call join which will pause the thread that we're on until the thread we're joining has completed and exited. The thread that we spawn could have an error like a panic or it could return a value that successfully back to the thread that joins it. So what we get from the join call is a result that wraps the possible success value returned from the thread or an error if the thread panicked.
    // wait until thread has exited
    handle.join().unwrap();

    // Threading is a bit heavyweight, creating a new thread allocates an operating system dependent amount of RAM for the threat zone stack, often a couple of megabytes. Whenever a CPU switches from running one thread to another. It has to do an expensive context switch, so the more threads you have trying to share a CPU core, the more overhead you will have in the context switching.

    // Even so, threads is a fantastic tool when you need to use CPU and memory concurrently because they can run simultaneoudly on multiple cores and actually accomplish more work.  However, if you just want to continue doing some work while you are waiting for something like this guy or network IO then I encourage you to look into async await. Which is a much more efficient approach for concurrently waiting for things.
}
