// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;

// pub fn sum(v: Vec<i32>) -> i32 {
//     let mid = v.len() /2;
//
//      let handle = std::thread::scope(|scope| {
//          let v_ref = &v;
//         let v1result = scope.spawn(move || {
//             let v1 = &v_ref[..mid];
//             v1.iter().sum::<i32>()
//         });
//
//         let v2result = scope.spawn(move || {
//             let v2 = &v_ref[mid..];
//             v2.iter().sum::<i32>()
//         });
//          v1result.join().unwrap() + v2result.join().unwrap()
//     });
//
//
//     handle
// }

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() /2;
    let (v1, v2) = v.split_at(mid);

    thread::scope(|scope| {
        let v1result = scope.spawn(|| { v1.iter().sum::<i32>() });
        let v2result = scope.spawn(|| { v2.iter().sum::<i32>() });
        v1result.join().unwrap() + v2result.join().unwrap()
    })
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
