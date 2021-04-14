mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use quicksort::Quicksort;
pub use selectionsort::SelectionSort;

pub trait Sorter {
    fn sort<T>(self, slice: &mut [T])
    where
        T: Ord;
}
