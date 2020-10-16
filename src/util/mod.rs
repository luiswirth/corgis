pub mod consts;
pub mod macros;

// inspiration by https://hole.tuziwo.info/dyn-iterator.html
// this is needed because of RFC 1522: AnyFn() -> impl Trait not allowed
pub struct DynIter<'iter, Item>(Box<dyn Iterator<Item = Item> + 'iter>);

impl<'iter, Item> DynIter<'iter, Item> {
    pub fn new<Iter>(iter: Iter) -> Self
    where
        Iter: Iterator<Item = Item> + 'iter,
    {
        Self(Box::new(iter))
    }
}
