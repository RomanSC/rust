//! fibonacci.rs | Sat, Feb 17, 2017 | Roman S. Collins

fn fib_x(n: usize, mut memo: Vec<usize>) -> usize {
    // if n in memo
    if memo.contains(n) {
        print!("DEBUG: memo contains n");
        n // return n
        // fib_e(0) == 0, fib_e(1) == 1
    } else if n == 0 || n == 1 {
        n // return n
    } else {
        // Without memoization this algorithm
        // works like this:
        // https://i.stack.imgur.com/59Rpw.png
        let n = fib_x(n - 1, memo) + fib_x(n - 2, memo);
        memo.push(n);
        n
    }
}

fn main() {
        let n: usize = 10;

        // Let's just make it global
        // https://doc.rust-lang.org/book/const-and-static.html
        //const mut memo: Vec<usize> = Vec::new(); // No fuck this

        //println!("{}", fib_e(n, memo));
        println!("{}", fib_x(n, memo));
}
