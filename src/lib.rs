extern crate typenum;
use typenum::marker_traits::{NonZero, Unsigned};
use typenum::operator_aliases::Add1;
use typenum::consts::{U1, U10000000000000000000};
use typenum::type_operators::{IsGreaterOrEqual, Max};
use std::iter::Iterator;
use std::marker::PhantomData;

#[allow(non_camel_case_types)]
type BIG_BOY = U10000000000000000000;

// unit struct definitions
pub struct Const();
pub struct LogN();
pub struct N<T: NonZero + Unsigned>(PhantomData<T>); // PhantomData<T> is the exponent
pub struct TwoToN();
pub struct NFactorial();
pub struct NToN();

pub trait LessComplexThan<T> {}
pub trait MaxComplexity<T> {
    type Output;
}
pub trait Complexity {}

// edge case of MaxComplexity where the exponent of N() is being incremented
pub trait IncrementN {
    type Output;
}

macro_rules! associative_max {
    ($greater:ty, $lesser:ty, $constraint:tt) => {
        impl<T: Unsigned + NonZero> MaxComplexity<$lesser> for $greater {
            type Output = $greater;
        }

        impl<T: Unsigned + NonZero> MaxComplexity<$greater> for $lesser {
            type Output = $greater;
        }
    };
    ($greater:ty, $lesser:ty) => {
        impl MaxComplexity<$lesser> for $greater {
            type Output = $greater;
        }

        impl MaxComplexity<$greater> for $lesser {
            type Output = $greater;
        }
    }
}

macro_rules! implement_ordering {
    ($greater:ty $(,$lesser:ty)+) => {
        $(
            associative_max!($greater, $lesser);
            impl LessComplexThan<$greater> for $lesser {}

        )*
        impl MaxComplexity<$greater> for $greater {
            type Output = $greater;
        }

        impl Complexity for $greater {}

        implement_ordering!($($lesser),+);
    };
    ($greater:ty) => {
        impl MaxComplexity<$greater> for $greater {
            type Output = $greater;
        }
        impl Complexity for $greater {}
    };
}

impl<T: Unsigned + NonZero, U: Unsigned + NonZero + IsGreaterOrEqual<T>> LessComplexThan<N<T>> for N<U> {}

impl<T: Unsigned + NonZero + Max<U>, U: Unsigned + NonZero> MaxComplexity<N<T>> for N<U>
    where <T as Max<U>>::Output: NonZero + Unsigned {
    type Output = N<<T as Max<U>>::Output>;
}

implement_ordering!(NToN, NFactorial, TwoToN, LogN, Const);

associative_max!(N<T>, Const, T);
associative_max!(N<T>, LogN, T);
associative_max!(TwoToN, N<T>, T);
associative_max!(NFactorial, N<T>, T);
associative_max!(NToN, N<T>, T);

pub struct O<S: Complexity, T> {
    fake: PhantomData<S>,
    real: T
}

#[allow(non_snake_case)]
pub fn create_O<S: Complexity, T>(real: T) -> O<S, T> {
    O { fake: PhantomData, real: real }
}

impl<S: Complexity, T: Iterator> Iterator for O<S, T> {
    type Item = O<S, T::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.real.next() {
            Some(x) => Some(create_O::<S, T::Item>(x)),
            None => None
        }
    }
}

