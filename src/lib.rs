mod bubblesort;

pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice)
}

#[cfg(test)]
mod tests {
    use crate::{sort, Sorter};

    struct StdSorter;

    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort()
        }
    }

    #[test]
    fn std_works() {
        let mut things = vec![4, 1, 3, 2];
        sort::<_, StdSorter>(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
