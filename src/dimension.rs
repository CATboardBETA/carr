pub trait Dimensions<const N_DIMS: usize> {
    const SHAPE: [usize; N_DIMS];
}

#[allow(unused_imports)]
use crate::size::Size;
#[allow(unused_imports)]
use paste::paste;

macro_rules! dim {
    ($name:ident; $count:literal; $($g:ident),*) => {
        #[allow(unused_parens)]
        #[derive(PartialEq, Eq)]
        pub struct $name<$($g: Size),*> {
            phantom: std::marker::PhantomData<($($g),*)>
        }
        impl<$($g: Size),*> Dimensions<$count> for $name<$($g),*> {
            const SHAPE: [usize; $count] = [$($g::SIZE),*];
        }
    };
}

dim!(D0; 0;);
dim!(D1; 1; S1);
dim!(D2; 2; S1, S2);
dim!(D3; 3; S1, S2, S3);
dim!(D4; 4; S1, S2, S3, S4);
dim!(D5; 5; S1, S2, S3, S4, S5);
dim!(D6; 6; S1, S2, S3, S4, S5, S6);
dim!(D7; 7; S1, S2, S3, S4, S5, S6, S7);
dim!(D8; 8; S1, S2, S3, S4, S5, S6, S7, S8);
dim!(D9; 9; S1, S2, S3, S4, S5, S6, S7, S8, S9);
dim!(D10; 10; S1, S2, S3, S4, S5, S6, S7, S8, S9, S10);
dim!(D11; 11; S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11);
dim!(D12; 12; S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12);

#[derive(Eq, PartialEq)]
pub struct Transpose<X: Dimensions<2>>(X);
impl<X: Dimensions<2>> Dimensions<2> for Transpose<X> {
    const SHAPE: [usize; 2] = [X::SHAPE[1], X::SHAPE[0]];
}
