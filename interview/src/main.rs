
fn max_in_slice(slice: &[i32]) -> Option<i32> {
    slice.iter().copied().max()
}

use std::collections::HashMap;

fn freq_count(v: &[i32]) -> HashMap<i32,usize> {
    let mut m = HashMap::new();
    for &x in v { *m.entry(x).or_insert(0) +=1};
    m
}


// fn parse_csv_line_splt_ch(line: &str) -> std::str::Split<'_, char> {
//     line.split(',')
// }

// fn parse_csv_line(line: &str) -> std::iter::Map<std::str::Split<'_, char>, fn(&str) -> &str>
// {
//     line.split(',').map(|s| s.trim())
// }

fn parse_csv_line(line: &str) -> Vec<String>
{
    line.split(',').map(|s| s.trim().to_string()).collect()
}

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn find_even(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().find(|x| x % 2 == 0)
}

// fn even_iter(v: Vec<i32>) -> impl Iterator<Item = i32> {
//     v.into_iter().filter(|x| x % 2 == 0)
// }

fn find_even_number(slice: &[i32]) -> Option<i32> {
    for &s in slice {
        if s % 2 == 0 {
            return Some(s);
        }
    }
    None
}

// use std::collections::HashMap;
// fn freq(v: &[i32]) -> HashMap<i32, usize> {
//     let mut m = HashMap::new();
//     for &x in v { *m.entry(x).or_insert(0) += 1; }
//     m
// }

fn main() {

    // 1. Swap two variables
    let mut a = 5;
    let mut b = 8;

    std::mem::swap(&mut a, &mut b);
    println!("a={}, b={}", a, b);

    // 2. Reverse a vector
    let mut v = vec![1,2,3];
    v.reverse();
    println!("{:?}", v);

    //.reverse() requires the vector to be mutable.

    // 3. Find the max in a slice

    let nums = [12,3,5];
    let max = max_in_slice(&nums);

    match max {
        Some(val) => println!("{}", val),
        None => println!("Slice is empty"),
    }

    // 4. Fibonacci (iterative)


    // 5. Parse a CSV line into Vec<String>
    let line = "  Alice , 24 , Developer , New York ";
    let fields = parse_csv_line(line);
    println!("{:?}", fields);

    // 6. First word of a string (\&str)
    let test = "hello world";
    let out = first_word(test);
    println!("{:?}", out);

    // 14. find even
    let number_array = [ 1, 5, 5, 8, 6, 7];
    let even = find_even(&number_array);

    match even {
        Some(val) => println!("{}", val),
        None => println!("no even numbers found"),
    }

    let even2 = find_even_number(&number_array);


    match even2 {
        Some(val) => println!("{}",val),
        None => println!("even number not found"),
    }

    let mut map = freq_count(&number_array);

    if let Some((key, value)) = map.iter().max_by_key(|entry| entry.1) {
        println!("Most frequent: {} appears {} times", key, value);
    }

    println!("{:?}", map);

    // let b = String::from("hello");
    // transfer_ownership(b);
    // println!("{}", b);

    // multiple immutable borrows
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("r1: {}, r2: {}", r1, r2);

    let mut s1 = String::from("hello");

    let r3 = &s1;
    let r4 = &mut s1;

    println!("r3: {}", r3); // ❌ Compile-time error
    println!("r4: {}", r4);

    //r1 = &s → someone is reading the document.
    //r2 = &mut s → someone is trying to edit the document.
}


fn transfer_ownership(s: String) {
    println!("{}", s);
}

// zero cost abstraction for concurrency in rust means you can write
// high level, egonomic concurrent code without paying a performance penalty compared to writing
// the same logic in low level code


// when you write concurrent code using rust's abstracvtions like channels, threads, Mutex, or async
//compiler ensures
//1. no hidden runtime overhead unless explicity introduced
// concurrecny safety is enforced at compile time so
// no need for garbage collector
// no runtime checks to prevent race conditions

// a merkle tree is a binary tree of hashes
// root, leaf, parent. use case: btc and eht use them to validate transaction
// sets in blocks


// simple blockchain in rust
//1. 

// struct Block {
//     index: u64,
//     timestamp: u64,
//     data: String,
//     prev_hash: String,
//     hash: String,
// }

// struct Blockchain {
//     chain: Vec<Block>,
// }

// impl Blockchain {
//     fn add_block(&mut self, data: String){
//         let prev = self.chain.last().unwrap();
//         let new_block = Block::new(prev.index + 1, data, prev.hash.clone());
//         self.chain.push(new_block);
//     }
// }