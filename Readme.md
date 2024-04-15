# Learn Rust!
#learnRust with me. 
I have created a concise, articulated, to-the-point #Rust crash course with practical examples and explanation with inline code comments.
this repository is for you hands-on-learners!

Why I created the course?

In this day and age, when we have all the information in the world at our fingertips, we often get confused when we want to learn or explore something new. Our minds gets flooded with all the information and the content which is very overwhelming for us.

I have created this #Rust crash #course just to deal with this and give you a glimpse into Rust with concise, practical examples and explanation with inline code comments.

Why learn rust?

#Rust takes your programing skills and multiplies with atleast 5. It gives you a new perspective to do things and makes you a better developer just by building things in rust. If you don’t want to code or have a career in Rust, then don't. But learning Rust will make you a far better dev and take your fundamentals and thinking to the next level with new perspectives of robust memory management, ownership, borrowing, concurrency, performance & speed.

## Install Rust

Rust is required for this course!  The latest stable version is always recommended.

- Go to [rust-lang.org](https://rust-lang.org) and click on the `Get Started`
   button and follow the instructions to install Rust for your operating system.
   - Please DO NOT install rust via some other package manager.  It will probably be a version that is _really old_.

You should get somewhat similar output if you run commands like the ones below (newer versions are okay).  If you 
already have an old version of Rust installed, then run `rustup update` to install a newer version.

```shell
$ rustc --version
rustc 1.54.0 (a178d0322 2021-07-26)
$ cargo --version
cargo 1.54.0 (5ae8d74b3 2021-06-22)
```

- Clone or download this repository to your computer.

## Prepare Your Development Environment

Please do the following (see the [How To Learn Rust](https://github.com/1xAhmed/rust_crash_course/blob/main/HowToLearnRust.md)
page for details on all of these)
- [ ] Choose an IDE (or Editor) and configure it with Rust support and customize it to your liking
  - **VS Code users**: Please use the [`rust-analyzer`] extension. **_If you have the `rust` extension installed, please uninstall it!_**
  - **IntelliJ users**: Please use the [`intellij-rust`] extension.
- [ ] Choose one place to "find answers" and either introduce yourself (if it's a forum, IRC, etc.) or find the answer
      to one question you have.
- [ ] Try doing something in Rust!  If you don't have a better idea, then just do this:
  - `cargo new hello`
  - `cd hello`
  - `cargo run`
  - Edit `src/main.rs` and change the message.
  - `cargo run` again to see your new message.
- [ ] Check out the descriptions of the tools and books.


# Exercises

Please clone this repository! These exercises are designed as Rust projects for you to edit on your
own computer, with the exception of Exercise A (which is just a `README.md` file).

The exercises are separate Rust projects.  For each exercise,
you should:
- Open the corresponding`EXERCISE_NAME` directory in your IDE/Editor
  - Seriously, just open the _individual exercise directory_ in your IDE. If you open the entire repository, your IDE will probably complain that it sees multiple Rust projects.
- Navigate to the same directory with your Terminal application (so you can run `cargo run`, etc.)
- Open up the `src/main.rs` file.
- Follow the numbered exercise instructions in the code comments.

If you encounter any problems with the exercises, please feel free to contact me. 😄

# Projects

- [Invaders](https://github.com/1xahmed/space_invaders) - A terminal-based Space Invaders arcade game clone.


[`rust-analyzer`]: https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer
[`intellij-rust`]: https://intellij-rust.github.io/
