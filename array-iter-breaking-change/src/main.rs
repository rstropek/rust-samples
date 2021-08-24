use std::marker::PhantomData;

struct MyArray<T> {
    phantom: PhantomData<T>,
}

// See also https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/iter/traits/collect.rs#L204
trait MyIntoIterator {
    type Item;
    type IntoIter: MyIterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}

// See also https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/iter/traits/iterator.rs#L55
trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// See also https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/array/mod.rs#L191
impl<'a, T> MyIntoIterator for &'a MyArray<T> {
    type Item = &'a T;
    type IntoIter = MyIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        MyIter::<'a, T>{ _phantom: &self.phantom }
    }
}

/*
// See also https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/array/mod.rs#L173
impl<T> MyIntoIterator for MyArray<T> {
    type Item = T;
    type IntoIter = MyIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        MyIntoIter::<T>{ _phantom: self.phantom }
    }
}

// See https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/array/iter.rs#L13
struct MyIntoIter<T> {
    _phantom: PhantomData<T>,
}

// See https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/array/iter.rs#L102
impl<T> MyIterator for MyIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
*/

// See also https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/slice/iter.rs#L65
struct MyIter<'a, T: 'a> {
    _phantom: &'a PhantomData<T>,
}

// See also https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/slice/iter.rs#L133
// and https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/slice/iter/macros.rs#L130
impl<'a, T: 'a> MyIterator for MyIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn main() {
    let _arr = MyArray::<i32>{ phantom: PhantomData::<i32>{} };

    let mut iter: MyIter<i32> = _arr.into_iter();
    let _first_item: Option<&i32> = iter.next();
}
