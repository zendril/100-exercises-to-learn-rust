// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() /2;
    let static_ref: &'static mut [i32] = v.leak();
    let v1 = &static_ref[..mid];
    let v2 = &static_ref[mid..];
    // let (v1, v2) = v.split_at(mid);
    // let v1 = v1.to_vec();
    // let v2 = v2.to_vec();

    let first_handle = thread::spawn(move || {
        v1.into_iter().sum::<i32>()
    });

    let second_handle = thread::spawn(move || {
        v2.into_iter().sum::<i32>()
    });


    first_handle.join().unwrap() + second_handle.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
