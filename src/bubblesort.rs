use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T: Ord>(self, slice: &mut [T]) {
        if slice.len() == 0 {
            return;
        }

        let mut swapped = true;
        while swapped {
            swapped = false;

            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BubbleSort, Sorter};

    #[test]
    fn it_works() {
        let mut things = vec![4, 1, 3, 2, 5];
        BubbleSort.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }
}
