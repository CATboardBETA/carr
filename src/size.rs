use std::ops::Deref;

pub trait Size {
    const SIZE: usize;
}

#[derive(PartialEq, Eq)]
pub struct S<const N: usize>;

impl<const N: usize> Deref for S<N> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &N
    }
}

impl<const N: usize> Size for S<N> {
    const SIZE: usize = N;
}

macro_rules! op2 {
    ($name:ident; $p1:ident $p2:ident; $op_fn:tt) =>  {
        #[allow(dead_code)]
        pub struct $name<$p1: Size, $p2: Size>($p1, $p2);

        impl<$p1: Size, $p2: Size> std::ops::Deref for $name<$p1, $p2> {
            type Target = usize;
            fn deref(&self) -> &Self::Target {
                &Self::SIZE
            }
        }

        impl<$p1: Size, $p2: Size> Size for $name<$p1, $p2> {
            const SIZE: usize = $p1::SIZE $op_fn $p2::SIZE;
        }

        impl<$p1: Size, $p2: Size> $name<$p1, $p2> {
            #[allow(dead_code)]
            #[must_use]
            const fn size(&self) -> usize {
                return Self::SIZE
            }
        }
    };
}

op2!(Add; X Y; +);
op2!(Sub; X Y; -);
op2!(Mul; X Y; *);
op2!(Div; X Y; /);
op2!(Mod; X Y; %);

#[cfg(test)]
mod test {
    use super::*;
    use crate::size::S;

    #[test]
    fn add() {
        let x = S::<11>;
        let y = S::<2>;
        assert_eq!(13, Add(x, y).size());
    }

    #[test]
    fn sub() {
        let x = S::<11>;
        let y = S::<2>;
        assert_eq!(9, Sub(x, y).size());
    }

    #[test]
    fn mul() {
        let x = S::<11>;
        let y = S::<2>;
        assert_eq!(22, Mul(x, y).size());
    }

    #[test]
    fn div() {
        let x = S::<11>;
        let y = S::<2>;
        assert_eq!(5, Div(x, y).size());
    }

    #[test]
    fn modulo() {
        let x = S::<11>;
        let y = S::<2>;
        assert_eq!(1, Mod(x, y).size());
    }
}
