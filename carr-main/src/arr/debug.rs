//! Print an array. [`Debug`] and [`Display`] are currently the same. [`Debug`]'s implementation
//! should not be relied upon.

use crate::arr::Arr;
use crate::backend::Backend;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};

impl<B, T, const D: &'static [usize]> Display for Arr<B, T, D>
where
    B: Backend<T> + Clone,
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dims_left = D.len();
        if dims_left > 2 {
            while dims_left > 2 {
                write!(f, "[")?;
                dims_left -= 1;
            }
            writeln!(f)?;
        }

        if dims_left == 1 {
            writeln!(
                f,
                "[{}]",
                self.storage()
                    .clone()
                    .index_range(0..D.iter().product())
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        } else {
            // dims_left == 2
            writeln!(f, "[")?;
            writeln!(
                f,
                "{}",
                self.storage()
                    .clone()
                    .index_range(0..D.iter().product())
                    .chunks(D[D.len() - 2])
                    .into_iter()
                    .map(|x| {
                        format!(
                            "  [{}]",
                            x.map(ToString::to_string).collect::<Vec<_>>().join(", ")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            )?;
            writeln!(f, "]")?;
        }
        if dims_left < D.len() {
            while dims_left < D.len() {
                write!(f, "]")?;
                dims_left += 1;
            }
        }
        Ok(())
    }
}

impl<B, T, const D: &'static [usize]> Debug for Arr<B, T, D>
where
    B: Backend<T> + IntoIterator<Item = T> + Clone,
    T: Default + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self}")
    }
}
