use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut fd = File::open("2600-0.txt").expect("can't do it bro");
    let mut contents = String::new();

    fd.read_to_string(&mut contents).expect(
        "can't read the file",
    );

    // for word in contents.split_whitespace() {
    //     println!("{}", word);
    // }

    // println!("{}", contents.split_whitespace().count())

    //    println!("{}", contents);

    let words = contents.split_whitespace();

    // let mut sorted: Vec<&str> = words.collect();
    // sorted.sort();

    // let mut uniq = BTreeSet::new();

    // let uniqwords = words.fold(uniq, |mut acc, x| {
    //     acc.insert(x);
    //     acc
    // });

    let mut hash: HashMap<&str, usize> = HashMap::new();
    for word in words {
        let count = hash.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", hash);

    let mut sorted: Vec<(&&str, &usize)> = hash.iter().collect();
    sorted.sort_by(|&(_, &l), &(_, &r)| r.cmp(&l));

    println!("{:?}", sorted);
}

#[derive(PartialEq)]
struct Foo(usize);

// impl PartialEq for Foo {
//     fn eq(&self, rhs: &Self) -> bool {
//         let &Foo(l) = self;
//         let &Foo(r) = rhs;
//         l == r
//     }
// }
