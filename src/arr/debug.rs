use crate::arr::Arr;
use crate::backend::Backend;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};

impl<B: Backend<T> + IntoIterator<Item = T> + Clone, T: Display, const N_DIMS: usize> Display
    for Arr<B, T, N_DIMS>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dims_left = N_DIMS;
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
                    .into_iter()
                    .map(|x| x.to_string())
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
                    .into_iter()
                    .chunks(self.dims.shape()[N_DIMS - 2])
                    .into_iter()
                    .map(|x| {
                        format!(
                            "  [{}]",
                            x.map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            )?;
            writeln!(f, "]")?;
        }
        if dims_left < N_DIMS {
            while dims_left < N_DIMS {
                write!(f, "]")?;
                dims_left += 1;
            }
        }
        Ok(())
    }
}

impl<B: Backend<T> + IntoIterator<Item = T> + Clone, T: Display, const N_DIMS: usize> Debug
    for Arr<B, T, N_DIMS>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self}")
    }
}
