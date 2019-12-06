use itertools::{multipeek, MultiPeek};

#[derive(Clone, Debug)]
pub struct EliminateRuns<I>
where
    I: Iterator,
    <I as Iterator>::Item: Clone,
    <I as Iterator>::Item: std::fmt::Debug,
{
    iter: MultiPeek<I>,
}

pub fn eliminate_runs<I>(iterable: I) -> EliminateRuns<I::IntoIter>
where
    I: IntoIterator,
    <I as IntoIterator>::Item: Clone,
    <I as IntoIterator>::Item: std::fmt::Debug,
{
    EliminateRuns {
        iter: multipeek(iterable),
    }
}

impl<I> Iterator for EliminateRuns<I>
where
    I: Iterator,
    <I as Iterator>::Item: Clone,
    <I as Iterator>::Item: std::fmt::Debug,
    <I as Iterator>::Item: Eq,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.iter.next() {
            Some(x) => match self.iter.peek().cloned() {
                Some(peek) => {
                    if x == peek {
                        match self.iter.peek().cloned() {
                            Some(peek3) => {
                                if x == peek3 {
                                    self.iter.next();
                                    self.iter.next();

                                    while self.iter.peek().cloned() == Some(peek3.clone()) {
                                        self.iter.next();
                                    }

                                    return self.iter.next();
                                } else {
                                    return Some(x);
                                }
                            }
                            None => return Some(x),
                        };
                    }

                    Some(x)
                }
                None => Some(x),
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_duplicates() {
        let example = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(example.len(), eliminate_runs(example).count());
    }

    #[test]
    fn duplicates_no_runs() {
        let example = vec![0, 1, 1, 2, 2, 3, 3, 4, 5, 6, 6];
        assert_eq!(example.len(), eliminate_runs(example).count());
    }

    #[test]
    fn beginning_run() {
        let example = vec![1, 1, 1, 2, 3, 4];
        assert_eq!(3, eliminate_runs(example).count());
    }

    #[test]
    fn middle_run() {
        let example = vec![1, 2, 3, 4, 4, 4, 5, 6];
        assert_eq!(5, eliminate_runs(example).count());
    }

    #[test]
    fn end_run() {
        let example = vec![0, 1, 2, 3, 3, 3];
        assert_eq!(3, eliminate_runs(example).count());
    }
}
