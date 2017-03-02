use std::marker::PhantomData;

pub trait Finite {
    type Item;

    fn len(&self) -> usize;
    fn index(&self, i: usize) -> Self::Item;

    fn empty() -> Empty<Self::Item> {
        Empty(PhantomData)
    }

    fn singleton<T>(t: T) -> Singleton<T> {
        Singleton(t)
    }

    fn union<B>(self, other: B) -> Union<Self, B>
        where Self: Sized, B: Finite<Item=Self::Item>
    {
        Union(self, other)
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
        Product(self, other)
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

impl<T> Finite for Empty<T> {
    type Item = T;

    fn len(&self) -> usize {
        0
    }

    fn index(&self, _: usize) -> T {
        panic!("cannot index empty set")
    }
}

pub struct Singleton<T>(pub T);

impl<'a, T> Finite for &'a Singleton<T> {
    type Item = &'a T;

    fn len(&self) -> usize {
        1
    }

    fn index(&self, i: usize) -> &'a T {
        match i {
            0 => &self.0,
            i => panic!("cannot access index {} in singleton set", i)
        }
    }
}

pub struct Union<A, B>(pub A, pub B);

impl<A, B> Finite for Union<A, B> where A: Finite, B: Finite<Item=A::Item> {
    type Item = A::Item;

    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    fn index(&self, i: usize) -> Self::Item {
        if i < self.0.len() {
            self.0.index(i)
        } else {
            self.1.index(i - self.0.len())
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

pub struct Product<A, B>(pub A, pub B);

impl<A, B> Finite for Product<A, B> where A: Finite, B: Finite {
    type Item = (A::Item, B::Item);

    fn len(&self) -> usize {
        self.0.len() * self.1.len()
    }

    fn index(&self, i: usize) -> Self::Item {
        let q = i / self.1.len();
        let r = i % self.1.len();
        (self.0.index(q), self.1.index(r))
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
    #[test]
    fn it_works() {
    }
}
