//! Integration with the [image crate](https://lib.rs/crates/image).

use crate::integrations::IntegrationError;
use crate::prelude::*;
use image::{ImageBuffer, Pixel};
use std::marker::PhantomData;
use std::ops::Deref;

impl<P, B, const D: &'static [usize]> TryFrom<ImageBuffer<P, B>> for Arr<B, P::Subpixel, D>
where
    P: Pixel,
    B: Backend<P::Subpixel> + Deref<Target = [P::Subpixel]>,
{
    type Error = IntegrationError;

    fn try_from(value: ImageBuffer<P, B>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(IntegrationError::EmptyArr)
        } else if value.len() == D.iter().product() {
            Ok(Arr {
                storage: value.into_raw(),
                _phantom: PhantomData,
            })
        } else {
            Err(IntegrationError::DimMismatch)
        }
    }
}

impl<P, B, const D: &'static [usize]> TryFrom<Arr<B, P::Subpixel, D>> for ImageBuffer<P, B>
where
    P: Pixel,
    B: Backend<P::Subpixel> + Deref<Target = [P::Subpixel]>,
{
    type Error = IntegrationError;

    fn try_from(value: Arr<B, P::Subpixel, D>) -> Result<Self, Self::Error> {
        const {
            assert!(
                D.len() == 3 && P::CHANNEL_COUNT as usize == D[2],
                "Converting from an `Arr` to an image requires the array be in the shape [y, x, subpixel]"
            );
        }

        Ok(Self::from_raw(
            D[1].try_into().map_err(|_| IntegrationError::DimOverflow)?,
            D[0].try_into().map_err(|_| IntegrationError::DimOverflow)?,
            value.storage,
        )
        .unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::integrations::IntegrationError;
    use crate::ix;
    use crate::prelude::Arr;
    use image::{Rgb, RgbImage};

    #[test]
    fn dim_mismatch() {
        let image = RgbImage::new(16, 16);
        assert_eq!(
            image.try_into(),
            Err::<Arr<_, u8, { &[16, 15] }>, _>(IntegrationError::DimMismatch)
        );
    }

    #[test]
    fn empty() {
        let image = RgbImage::new(0, 0);
        assert_eq!(
            dbg!(image.clone().try_into()),
            Err::<Arr<_, u8, { &[] }>, _>(IntegrationError::EmptyArr)
        );
        assert_eq!(
            dbg!(image.clone().try_into()),
            Err::<Arr<_, u8, { &[0] }>, _>(IntegrationError::EmptyArr)
        );
        assert_eq!(
            dbg!(image.clone().try_into()),
            Err::<Arr<_, u8, { &[1, 2, 3, 4, 5] }>, _>(IntegrationError::EmptyArr)
        );
        assert_eq!(
            dbg!(image.try_into()),
            Err::<Arr<_, u8, { &[0, 1, 2, 3, 4] }>, _>(IntegrationError::EmptyArr)
        );
    }

    #[test]
    fn okay_1() {
        let image = RgbImage::new(8, 8);
        let expected = Arr::<Vec<u8>, u8, { &[8, 8, 3] }>::new_zeroed();
        assert_eq!(image.try_into(), Ok(expected));
    }

    #[test]
    fn okay_2() {
        let mut image = RgbImage::new(8, 8);
        image.put_pixel(1, 0, Rgb([0, 0, 255]));
        let mut expected = Arr::<Vec<u8>, u8, { &[8, 8, 3] }>::new_zeroed();
        *ix!(&mut expected => mut [0, 1, 2]).item() = 255;
        assert_eq!(image.try_into(), Ok(expected));
    }

    #[test]
    fn from_arr_1() {
        let mut a = Arr::<Vec<u8>, u8, { &[8, 8, 3] }>::new_zeroed();
        *ix!(&mut a => mut [0, 1, 2]).item() = 255;
        let mut expected = RgbImage::new(8, 8);
        expected.put_pixel(1, 0, Rgb([0, 0, 255]));
        assert_eq!(a.try_into(), Ok(expected));
    }
}
