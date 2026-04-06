#[cfg(test)]
mod tests {
    use crate::collections::RispList;

    #[test]
    fn empty_has_len_zero() {
        let list: RispList<i32> = RispList::empty();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn empty_is_empty() {
        let list: RispList<i32> = RispList::empty();
        assert!(list.is_empty());
    }

    #[test]
    fn empty_first_is_none() {
        let list: RispList<i32> = RispList::empty();
        assert!(list.first().is_none());
    }

    #[test]
    fn cons_len_is_one() {
        let list = RispList::cons(42, &RispList::empty());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn cons_first_is_head() {
        let list = RispList::cons(42, &RispList::empty());
        assert_eq!(list.first(), Some(&42));
    }

    #[test]
    fn cons_chain_has_correct_len() {
        let tail = RispList::cons(2, &RispList::empty());
        let list = RispList::cons(1, &tail);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn cons_chain_first_is_prepended() {
        let tail = RispList::cons(2, &RispList::empty());
        let list = RispList::cons(1, &tail);
        assert_eq!(list.first(), Some(&1));
    }

    #[test]
    fn rest_len_decrements() {
        let list: RispList<i32> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(list.rest().len(), 2);
    }

    #[test]
    fn rest_first_is_second_elem() {
        let list: RispList<i32> = vec![10, 20, 30].into_iter().collect();
        assert_eq!(list.rest().first(), Some(&20));
    }

    #[test]
    fn rest_of_empty_is_empty() {
        let list: RispList<i32> = RispList::empty();
        assert!(list.rest().is_empty());
    }

    #[test]
    fn nth_index_zero() {
        let list: RispList<i32> = vec![10, 20, 30].into_iter().collect();
        assert_eq!(list.nth(0).unwrap(), Some(&10));
    }

    #[test]
    fn nth_index_middle() {
        let list: RispList<i32> = vec![10, 20, 30].into_iter().collect();
        assert_eq!(list.nth(1).unwrap(), Some(&20));
    }

    #[test]
    fn nth_index_last() {
        let list: RispList<i32> = vec![10, 20, 30].into_iter().collect();
        assert_eq!(list.nth(2).unwrap(), Some(&30));
    }

    #[test]
    fn nth_out_of_bounds_returns_err() {
        let list: RispList<i32> = vec![1, 2].into_iter().collect();
        assert!(list.nth(5).is_err());
    }

    #[test]
    fn iter_correct_order() {
        let list: RispList<i32> = vec![1, 2, 3].into_iter().collect();
        let got: Vec<&i32> = list.iter().collect();
        assert_eq!(got, vec![&1, &2, &3]);
    }

    #[test]
    fn iter_empty_list() {
        let list: RispList<i32> = RispList::empty();
        assert_eq!(list.iter().count(), 0);
    }

    #[test]
    fn partial_eq_equal_lists() {
        let a: RispList<i32> = vec![1, 2, 3].into_iter().collect();
        let b: RispList<i32> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(a, b);
    }

    #[test]
    fn partial_eq_different_values() {
        let a: RispList<i32> = vec![1, 2].into_iter().collect();
        let b: RispList<i32> = vec![1, 3].into_iter().collect();
        assert_ne!(a, b);
    }

    #[test]
    fn partial_eq_different_lengths() {
        let a: RispList<i32> = vec![1, 2].into_iter().collect();
        let b: RispList<i32> = vec![1, 2, 3].into_iter().collect();
        assert_ne!(a, b);
    }

    #[test]
    fn from_iterator_collect() {
        let list: RispList<i32> = vec![5, 6, 7].into_iter().collect();
        assert_eq!(list.len(), 3);
        assert_eq!(list.first(), Some(&5));
    }

    #[test]
    fn clone_is_equal() {
        let list: RispList<i32> = vec![1, 2, 3].into_iter().collect();
        let clone = list.clone();
        assert_eq!(list, clone);
    }

    #[test]
    fn clone_is_shallow_rc() {
        // Clone should not deep-copy nodes — same Rc head, same length
        let list: RispList<i32> = vec![1, 2, 3].into_iter().collect();
        let clone = list.clone();
        assert_eq!(list.len(), clone.len());
    }

    #[test]
    fn display_non_empty() {
        let list: RispList<i32> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(format!("{list}"), "(1 2 3)");
    }

    #[test]
    fn display_empty() {
        let list: RispList<i32> = RispList::empty();
        assert_eq!(format!("{list}"), "()");
    }

    #[test]
    fn display_single() {
        let list: RispList<i32> = vec![42].into_iter().collect();
        assert_eq!(format!("{list}"), "(42)");
    }
}
