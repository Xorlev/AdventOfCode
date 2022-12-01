use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub trait TopK: Iterator {
    /// Calculates a rolling Top-K set of items from an iterator in constant space.
    /// Returns a sorted vector of the top-k elements, sorted descending.
    fn topk(self, k: usize) -> Vec<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
    {
        let mut heap = BinaryHeap::<Reverse<Self::Item>>::with_capacity(k + 1);

        self.for_each(|item| {
            heap.push(Reverse(item));

            if heap.len() == k + 1 {
                heap.pop();
            }
        });

        let mut vec: Vec<Self::Item> = itertools::unfold(heap, |heap| heap.pop().map(|item| item.0)).collect();
        vec.reverse();
        vec
    }
}

impl<T: ?Sized> TopK for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::TopK;

    #[test]
    fn it_works() {
        let seq = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expected_vec = vec![10, 9, 8, 7, 6];
        let expected: Vec<&i32> = expected_vec.iter().collect();
        let actual: Vec<&i32> = seq.iter().topk(5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_reversed() {
        let mut seq = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        seq.reverse();

        let expected_vec = vec![10, 9, 8, 7, 6];
        let expected: Vec<&i32> = expected_vec.iter().collect();
        let actual: Vec<&i32> = seq.iter().topk(5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_shuffled() {
        let mut seq = vec![2, 3, 4, 5, 6, 10, 7, 8, 9, 1];

        seq.reverse();

        let expected_vec = vec![10, 9, 8, 7, 6];
        let expected: Vec<&i32> = expected_vec.iter().collect();
        let actual: Vec<&i32> = seq.iter().topk(5);

        assert_eq!(actual, expected);
    }
}
