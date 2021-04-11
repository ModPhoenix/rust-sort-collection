use std::fmt::{Debug, Display};

use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T: Display + Ord + Debug>(slice: &mut [T]) {
        for unsorted in 1..slice.len() {
            let mut sorted = unsorted - 1;
            loop {
                if slice[sorted + 1] < slice[sorted] {
                    slice.swap(sorted, sorted + 1);

                    if sorted > 0 {
                        sorted -= 1;
                    }
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InsertionSort, Sorter};

    #[test]
    fn it_works() {
        let mut things = vec![25, 4, 1, 3, 5, 2, 6, 9];
        InsertionSort::sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5, 6, 9, 25]);
    }
}
