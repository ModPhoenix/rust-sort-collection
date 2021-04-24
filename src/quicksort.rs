use super::Sorter;
use std::fmt::Debug;
pub struct Quicksort;

impl Sorter for Quicksort {
    fn sort<T: Ord + Clone + Debug>(self, slice: &mut [T]) {
        if slice.len() == 0 {
            return;
        }

        quicksort(slice, 0, slice.len() - 1);
    }
}

fn quicksort<T: Ord + Clone + Debug>(slice: &mut [T], low: usize, high: usize) {
    if low < high {
        let p = partition(slice, low, high);

        quicksort(slice, low, p);
        quicksort(slice, p + 1, high);
    }
}

fn partition<T: Ord + Clone + Debug>(slice: &mut [T], low: usize, high: usize) -> usize {
    let mut i = low;
    let mut j = high;
    let pivot = &slice[(high + low) / 2].clone();

    let p: usize = loop {
        while &slice[i] < &pivot {
            i += 1;
        }

        while &slice[j] > &pivot {
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
    fn sorts_same_items() {
        let mut things = vec![5, 5, 5, 5, 5, 5, 5, 5, 5];
        Quicksort.sort(&mut things);
        assert_eq!(things, &[5, 5, 5, 5, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn it_works() {
        let mut things = vec![25, 4, 1, 3, 31, 5, 1, 4, 4, 2, 2, 6, 4, 9, 1];
        Quicksort.sort(&mut things);
        assert_eq!(things, &[1, 1, 1, 2, 2, 3, 4, 4, 4, 4, 5, 6, 9, 25, 31]);
    }

    #[test]
    fn it_works2() {
        let mut things = vec![6, 9, 8, 0, 5, 3, 5, 0, 6, 0];
        Quicksort.sort(&mut things);
        assert_eq!(things, &[0, 0, 0, 3, 5, 5, 6, 6, 8, 9]);
    }

    #[test]
    fn it_works3() {
        let mut things = vec![9, 3, 2, 4, 8, 4, 9, 2, 3, 7];
        Quicksort.sort(&mut things);
        assert_eq!(things, &[2, 2, 3, 3, 4, 4, 7, 8, 9, 9]);
    }
}
