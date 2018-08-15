extern crate typenum;
use typenum::marker_traits::{NonZero, Unsigned};
use typenum::type_operators::IsGreater;
use typenum::bit::B1;

// unit struct definitions
struct Const();
struct LogN();
struct N<T: NonZero + Unsigned>(std::marker::PhantomData<T>); // PhantomData<T> is the exponent
struct TwoToN();
struct NFactorial();
struct NToN();

trait LessComplexThan<T> {}
trait HigherComplexity<T> {
    type Output;
}

macro_rules! implement_ordering {
    ($greater:ident $(,$lesser:ident)+) => {
        $(
            impl HigherComplexity<$lesser> for $greater {
                type Output = $greater;
            }

            impl HigherComplexity<$greater> for $lesser {
                type Output = $greater;
            }

            impl HigherComplexity<$greater> for $greater {
                type Output = $greater;
            }

            impl LessComplexThan<$greater> for $lesser {}
        )*
        implement_ordering!($($lesser),+);
    };
    ($greater:ident) => {
        impl HigherComplexity<$greater> for $greater {
            type Output = $greater;
        }
    };
}

impl<T: NonZero + Unsigned, U: NonZero + Unsigned + IsGreater<T,Output=B1>> LessComplexThan<N<T>> for N<U> {}

impl<T: NonZero + Unsigned, U: NonZero + Unsigned + IsGreater<T,Output=B1>> HigherComplexity<N<T>> for N<U> {
    type Output = N<U>;
}


impl<T: NonZero + Unsigned, U: NonZero + Unsigned + IsGreater<T,Output=B1>> HigherComplexity<N<U>> for N<T> {
    type Output = N<U>;
}

implement_ordering!(NToN, NFactorial, TwoToN, N, LogN, Const);
