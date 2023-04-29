// fn unique(mut a: Vec<i32>) -> Vec<i32> {
//     // a.sort();
//     a.sort_unstable();
//     a.dedup();
//     a
// }

// HashSet
// use std::collections::HashSet;
// fn unique(a: Vec<i32>) -> Vec<i32> {
//     let mut set: HashSet<i32> = HashSet::new();
//     let mut ans = vec![];
//     for x in a {
//         if !set.contains(&x) {
//             set.insert(x);
//             ans.push(x);
//         }
//     }
//     ans
// }

// advanced 1: use generic types
// fn unique<T: Ord>(mut a: Vec<T>) -> Vec<T> {
//     // a.sort();
//     a.sort_unstable();
//     a.dedup();
//     a
// }

// advanced 2: keep items in order
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

// advanced 3: use iterators
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::IntoIterator;
fn unique<T>(a: impl IntoIterator<Item=T>) -> Vec<T>
where T: Eq + Hash + Copy
{
    let mut set: HashSet<T> = HashSet::new();
    let mut ans = Vec::new();
    for x in a {
        if !set.contains(&x) {
            set.insert(x);
            ans.push(x);
        }
    }
    ans
}

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}

#[test]
fn empty_list() {
    let input: Vec<i32> = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let mut expected_output = vec![1, 4, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}
