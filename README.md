# Rust:

Meta repository for all my experiments with Rust, a programming language I'll be fluent in someday.

####Basics:

Currently working my way through [Rust by Example](http://rustbyexample.com/)

###Lists:

\#TODO: Working on implementing Linked and Doubly Linked list in Rust soon.

###Fibonacci:
[./rust/blob/master/basics/fibonacci.rs](https://github.com/RomanSC/rust/blob/master/basics/fibonacci.rs)

I've written a fib() algorithm that's really slow. Works kinda like [this](https://images.duckduckgo.com/iu/?u=http%3A%2F%2Fimages.viralnova.com%2F000%2F121%2F207%2Fdog-chasing-own-tail.gif&f=1).

\#TODO: Soon it will be memoized. As soon as I figure out how to pass different typed arguments
into my fib function to change memo inside the fib() function:

`fn fib_x(n: i32) -> i32 {
    let mut memo = Vec::new();

    if memo.contains(&n) {
        n
    } else if n == 0 || n == 1 {
        n
    } else {
        let n = fib_x(n - 1) + fib_x(n - 2);

        memo.push(n);

        n
    }
\}
\
\fn main() {
\
\    let n: i32 = 39;
\    let mut memo: Vec<i32> = Vec::new();
\
\    println!("{}", fib_x(n));
\}`

[My other repositories](https://github.com/RomanSC/repositories) | [Fibonacci in Python](https://raw.githubusercontent.com/RomanSC/algorithms/master/chapter-1/waysoffib.py)
