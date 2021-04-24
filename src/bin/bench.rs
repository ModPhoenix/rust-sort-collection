use sort_collection::*;

use rand::prelude::*;
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}

impl<T: Debug> std::fmt::Debug for SortEvaluator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.t)
    }
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));
    for &n in &[0, 1, 10, 100, 1000, 10000] {
        let mut values = Vec::with_capacity(n);

        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            })
        }
        for _ in 0..10 {
            values.shuffle(&mut rand);

            let took = bench(BubbleSort, &values, &counter);
            println!("{} {} {}", "BubbleSort", n, took);
            let took = bench(InsertionSort, &values, &counter);
            println!("{} {} {}", "InsertionSort", n, took);
            let took = bench(SelectionSort, &values, &counter);
            println!("{} {} {}", "SelectionSort", n, took);
            let took = bench(Quicksort, &values, &counter);
            println!("{} {} {}", "Quicksort", n, took);
        }
    }
}

fn bench<T: Ord + Clone + Debug, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> usize {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    sorter.sort(&mut values);
    let count = counter.get();

    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }

    count
}
