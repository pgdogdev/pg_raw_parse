// FIXME(sage): Use Option::into_flat_iter when stable
// (https://github.com/rust-lang/rust/issues/148441)
/// Create an iterator from an option of an iterator, while retaining
/// ExactSizeIterator and/or DoubleEndedIterator, which `.flatten` or
/// `.flat_map` would lose
pub(crate) fn to_flat_iter<A>(iter: Option<impl IntoIterator<IntoIter = A>>) -> OptionFlatten<A> {
    OptionFlatten(iter.map(IntoIterator::into_iter))
}

pub(crate) struct OptionFlatten<A>(Option<A>);

impl<A: Iterator> Iterator for OptionFlatten<A> {
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut()?.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0
            .as_ref()
            .map(|i| i.size_hint())
            .unwrap_or((0, Some(0)))
    }
}

impl<A: DoubleEndedIterator> DoubleEndedIterator for OptionFlatten<A> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.as_mut()?.next_back()
    }
}

impl<A: ExactSizeIterator> ExactSizeIterator for OptionFlatten<A> {}
