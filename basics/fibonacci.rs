//

// fn fib_e(n: i32, memo: Vec) -> (i32, Vec) {
//     // memo as a vec
//     //let mut memo = Vec::new();

//     // if n in memo
//     if memo.contains(&n) {
//         print!("DEBUG: memo contains n");
//         (n, memo) // return n
//     // fib_e(0) == 0, fib_e(1) == 1
//     } else if n == 0 || n == 1 {
//         (n, memo) // return n
//     } else {
//         // Without memoization this algorithm
//         // works like this:
//         // https://i.stack.imgur.com/59Rpw.png
//         let n = fib_e(n - 1) + fib_e(n - 2);

//         memo.push(n);

//         //for x in &memo { // Check memo
//             // Does not seem to memoize
//             // correctly...
//             //print!("{} ", x);
//         //}
//         (n, memo)
//     }
// }

fn fib_x(n: i32) -> i32 {
    // memo as a vec
    let mut memo = Vec::new();

    // if n in memo
    if memo.contains(&n) {
        print!("DEBUG: memo contains n");
        n // return n
        // fib_e(0) == 0, fib_e(1) == 1
    } else if n == 0 || n == 1 {
        n // return n
    } else {
        // Without memoization this algorithm
        // works like this:
        // https://i.stack.imgur.com/59Rpw.png
        let n = fib_x(n - 1) + fib_x(n - 2);

        memo.push(n);

        //for x in &memo { // Check memo
        // Does not seem to memoize
        // correctly...
        //print!("{} ", x);
        //}
        n
    }
}

fn main() {
    let n: i32 = 39;
    let mut memo: Vec<i32> = Vec::new();

    //println!("{}", fib_e(n, memo));
    println!("{}", fib_x(n));
}
