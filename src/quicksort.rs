use std::fmt::{Debug, Display};

use super::Sorter;

pub struct Quicksort;

impl Sorter for Quicksort {
    fn sort<T: Display + Ord + Debug>(slice: &mut [T]) {
        quicksort(slice, 0, slice.len() - 1);
    }
}

fn quicksort<T: Display + Ord + Debug>(slice: &mut [T], low: usize, high: usize) {
    if low < high {
        let p = partition(slice, low, high);

        quicksort(slice, low, p);
        quicksort(slice, p + 1, high);
    }
}

fn partition<T: Display + Ord + Debug>(slice: &mut [T], low: usize, high: usize) -> usize {
    let mut i = low;
    let mut j = high;

    let p: usize = loop {
        while &slice[i] < &slice[high - 1] {
            i += 1;
        }

        while &slice[j] > &slice[high - 1] {
            j -= 1;
        }

        if i >= j {
            break j;
        }

        slice.swap(i, j);

        i += 1;
        j -= 1;
    };

    return p;
}

#[cfg(test)]
mod tests {
    use super::{Quicksort, Sorter};

    #[test]
    fn it_works() {
        let mut things = vec![25, 4, 1, 3, 31, 5, 1, 4, 4, 2, 2, 6, 4, 9, 1];
        Quicksort::sort(&mut things);
        assert_eq!(things, &[1, 1, 1, 2, 2, 3, 4, 4, 4, 4, 5, 6, 9, 25, 31]);
    }

    #[test]
    fn sorts_same_items() {
        let mut things = vec![5, 5, 5, 5, 5, 5, 5, 5, 5];
        Quicksort::sort(&mut things);
        assert_eq!(things, &[5, 5, 5, 5, 5, 5, 5, 5, 5]);
    }
}
