/// Helper type to hide size hint of iterator.
#[allow(dead_code)]
#[derive(Debug)]
pub struct HideSize<I>(pub I);

impl<I: Iterator> Iterator for HideSize<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
