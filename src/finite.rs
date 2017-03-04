use std::marker::PhantomData;

pub trait Finite {
    type Item;

    fn len(&self) -> usize;
    fn index(&self, i: usize) -> Self::Item;

    fn union<B>(self, other: B) -> Union<Self, B>
        where Self: Sized, B: Finite<Item=Self::Item>
    {
        Union { left: self, right: other }
    }

    fn map<B, F>(self, f: F) -> Map<Self, F>
        where Self: Sized, F: Fn(Self::Item) -> B
    {
        Map {
            finite: self,
            f: f,
        }
    }

    fn product<B>(self, other: B) -> Product<Self, B>
        where Self: Sized, B: Finite
    {
        Product { left: self, right: other }
    }

    fn iter(&self) -> Iter<&Self> {
        Iter {
            finite: self,
            index: 0,
        }
    }
}

impl<'a, F: ?Sized + Finite> Finite for &'a F {
    type Item = F::Item;

    fn len(&self) -> usize {
        (*self).len()
    }

    fn index(&self, i: usize) -> Self::Item {
        (*self).index(i)
    }
}

pub struct Empty<T>(PhantomData<T>);

pub fn empty<T>() -> Empty<T> {
    Empty(PhantomData)
}

impl<T> Finite for Empty<T> {
    type Item = T;

    fn len(&self) -> usize {
        0
    }

    fn index(&self, _: usize) -> Self::Item {
        panic!("cannot index empty set")
    }
}

pub struct Union<A, B> {
    left: A,
    right: B,
}

impl<A, B> Finite for Union<A, B> where A: Finite, B: Finite<Item=A::Item> {
    type Item = A::Item;

    fn len(&self) -> usize {
        self.left.len() + self.right.len()
    }

    fn index(&self, i: usize) -> Self::Item {
        if i < self.left.len() {
            self.left.index(i)
        } else {
            self.right.index(i - self.left.len())
        }
    }
}

pub struct Map<A, F> {
    finite: A,
    f: F,
}

impl<A, F, U> Finite for Map<A, F> where A: Finite, F: Fn(A::Item) -> U {
    type Item = U;

    fn len(&self) -> usize {
        self.finite.len()
    }

    fn index(&self, i: usize) -> Self::Item {
        (self.f)(self.finite.index(i))
    }
}

pub struct Product<A, B> {
    left: A,
    right: B,
}

impl<A, B> Finite for Product<A, B> where A: Finite, B: Finite {
    type Item = (A::Item, B::Item);

    fn len(&self) -> usize {
        self.left.len() * self.right.len()
    }

    fn index(&self, i: usize) -> Self::Item {
        let q = i / self.right.len();
        let r = i % self.right.len();
        (self.left.index(q), self.right.index(r))
    }
}

pub fn singleton<T: Copy>(t: T) -> Singleton<T> {
    Singleton { inner: t }
}

pub struct Singleton<T> {
    inner: T,
}

impl<T: Copy> Finite for Singleton<T> {
    type Item = T;

    fn len(&self) -> usize {
        1
    }

    fn index(&self, i: usize) -> Self::Item {
        match i {
            0 => self.inner,
            i => panic!("cannot access index {} in singleton set", i)
        }
    }
}

pub fn lazy<F, R>(f: F) -> Lazy<F> where F: Fn() -> R {
    Lazy { inner: f }
}

pub struct Lazy<F> {
    inner: F,
}

impl<F, R> Finite for Lazy<F> where F: Fn() -> R {
    type Item = R;

    fn len(&self) -> usize {
        1
    }

    fn index(&self, i: usize) -> Self::Item {
        match i {
            0 => (self.inner)(),
            i => panic!("cannot access index {} in singleton set", i)
        }
    }
}

pub struct Natural(pub usize);

impl Finite for Natural {
    type Item = usize;

    fn len(&self) -> usize {
        self.0
    }

    fn index(&self, i: usize) -> Self::Item {
        i
    }
}

pub struct Iter<A> {
    finite: A,
    index: usize,
}

impl<A: Finite> Iterator for Iter<A> {
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;
        if i < self.finite.len() {
            self.index += 1;
            Some(self.finite.index(i))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.finite.len() - self.index;
        (size, Some(size))
    }
}

impl<A: Finite> ExactSizeIterator for Iter<A> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(empty::<()>().iter().len() == 0)
    }
}
