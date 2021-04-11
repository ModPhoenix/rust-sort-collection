use std::fmt::{Debug, Display};

use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T: Display + Ord + Debug>(slice: &mut [T]) {
        let mut min;
        let mut index: usize = 0;

        for i in 0..slice.len() {
            min = &slice[i];

            for j in i..slice.len() {
                if &slice[j] < &min {
                    min = &slice[j];
                    index = j;
                }
            }

            if &slice[i] != min {
                slice.swap(i, index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SelectionSort, Sorter};

    #[test]
    fn it_works() {
        let mut things = vec![25, 4, 1, 3, 5, 2, 6, 4, 9];
        SelectionSort::sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 4, 5, 6, 9, 25]);
    }
}
