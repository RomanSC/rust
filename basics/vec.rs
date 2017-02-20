// vec.rs | Sun, Feb 19, 2017 | Roman S. Collins
//
// Just testing passing around a vec

//fn pass_vec(memo: &mut Vec<i64>) -> &mut Vec<i64> {
fn pass_vec(memo: Vec<i64>) -> Vec<i64> {

    // print from pass_vec
    for i in memo {
        print!("{}", i)
    }
    memo
}

fn main() {
    let mut memo: Vec<i64> = Vec::new();

    memo.push(1);
    memo.push(2);
    memo.push(3);
    memo.push(4);
    memo.push(5);

    memo.pop();

    // print from main
    for i in memo {
        print!("{}", i);
    }

    //memo.shrink_to_fit()
    pass_vec(memo)

}
