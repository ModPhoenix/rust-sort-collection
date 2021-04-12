use std::fmt::{Debug, Display};

use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T: Display + Ord + Debug>(slice: &mut [T]) {
        for cursor in 0..slice.len() {
            let mut min_value_index = cursor;

            for rest_cursor in cursor..slice.len() {
                if &slice[rest_cursor] < &slice[min_value_index] {
                    min_value_index = rest_cursor;
                }
            }

            if &slice[cursor] != &slice[min_value_index] {
                slice.swap(cursor, min_value_index);
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
