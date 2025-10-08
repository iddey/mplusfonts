use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{Dimensions, OriginDimensions, Size};
use embedded_graphics::image::ImageDrawable;
use embedded_graphics::pixelcolor::{BinaryColor, Gray2, Gray4, Gray8, PixelColor};
use embedded_graphics::primitives::Rectangle;

use crate::color::{Colormap, Invert, Screen, WeightedAvg};
use crate::image::{Colors, Image, Mixed, SubImage};

/// Association with a colormap.
pub trait WithColormap<'a, T: Copy, const N: usize> {
    /// The output type.
    type Output<'b>
    where
        T: 'b,
        Self: 'a;

    /// Returns the association with a colormap, which then belongs to the image.
    fn with_colormap<'b>(&'a self, colormap: &'b Colormap<T, N>) -> Self::Output<'b>;
}

/// Association between an image and a colormap, which implements [`Mixed`] and allows for the
/// images being color-mixed to each use their own colormaps.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WithColormapImage<'a, 'b, U, T, const N: usize>
where
    T: PixelColor + Default + Invert + Screen + WeightedAvg,
    U: ImageDrawable + Colors<U::Color>,
{
    image: &'a Image<U>,
    colormap: &'b Colormap<T, N>,
}

/// Image with references to two overlapping image drawables and a set of two colormaps.
///
/// While also performing color conversion, drawing this image drawable involves mixing the colors
/// that form pairs of pixels in [`WeightedAvg`] blend mode.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WithColormapImageMix<'a, 'b, 'c, 'd, U, V, T, const N: usize>
where
    T: PixelColor + Default + Invert + Screen + WeightedAvg,
    U: ImageDrawable + Colors<U::Color>,
    V: ImageDrawable + Colors<V::Color>,
{
    first: Image<SubImage<'a, U>>,
    second: Image<SubImage<'b, V>>,
    colormap: &'c Colormap<T, N>,
    other_colormap: &'d Colormap<T, N>,
    area: Rectangle,
}

impl<'a, U, T: Copy, const N: usize> WithColormap<'a, T, N> for Image<U>
where
    T: PixelColor + Default + Invert + Screen + WeightedAvg,
    U: ImageDrawable + Colors<U::Color>,
{
    type Output<'b>
        = WithColormapImage<'a, 'b, U, T, N>
    where
        T: 'b,
        Self: 'a;

    fn with_colormap<'b>(&'a self, colormap: &'b Colormap<T, N>) -> Self::Output<'b> {
        Self::Output {
            image: self,
            colormap,
        }
    }
}

impl<'a, 'b, 'c, 'd, U, V, T, const N: usize> WithColormapImageMix<'a, 'b, 'c, 'd, U, V, T, N>
where
    T: PixelColor + Default + Invert + Screen + WeightedAvg,
    U: ImageDrawable + Colors<U::Color>,
    V: ImageDrawable + Colors<V::Color>,
{
    /// Creates a new image drawable with two pre-cut image drawables and the specified set of two
    /// colormaps.
    const fn new(
        first: Image<SubImage<'a, U>>,
        second: Image<SubImage<'b, V>>,
        colormap: &'c Colormap<T, N>,
        other_colormap: &'d Colormap<T, N>,
        area: Rectangle,
    ) -> Self {
        Self {
            first,
            second,
            colormap,
            other_colormap,
            area,
        }
    }
}

impl<U, V, T, const N: usize> OriginDimensions for WithColormapImageMix<'_, '_, '_, '_, U, V, T, N>
where
    T: PixelColor + Default + Invert + Screen + WeightedAvg,
    U: ImageDrawable + Colors<U::Color>,
    V: ImageDrawable + Colors<V::Color>,
{
    fn size(&self) -> Size {
        self.area.size
    }
}

macro_rules! impl_drawable {
    (
        $(
            $color_type:ty, $array_length:literal,
        )*
    ) => {
        $(
            impl<U, V, T> Drawable for WithColormapImageMix<'_, '_, '_, '_, U, V, T, $array_length>
            where
                T: PixelColor + Default + Invert + Screen + WeightedAvg,
                U: ImageDrawable<Color = $color_type> + Colors<$color_type>,
                V: ImageDrawable<Color = $color_type> + Colors<$color_type>,
            {
                type Color = T;
                type Output = ();

                fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
                where
                    D: DrawTarget<Color = Self::Color>,
                {
                    let first = self
                        .first
                        .colors()
                        .into_iter()
                        .map(|color| self.colormap.get(color));

                    let second = self
                        .second
                        .colors()
                        .into_iter()
                        .map(|color| self.other_colormap.get(color));

                    let start = self.colormap.first();
                    let end = self.colormap.last();
                    let other_start = self.other_colormap.first();
                    let other_end = self.other_colormap.last();
                    let colors = first
                        .zip(second)
                        .map(|(first, second)| {
                            first.weighted_avg(
                                second,
                                start,
                                end,
                                other_start,
                                other_end)
                        });

                    target.fill_contiguous(&self.area, colors)
                }
            }
        )*
    }
}

impl_drawable! {
    BinaryColor, 2,
    Gray2, 4,
    Gray4, 16,
    Gray8, 256,
}

macro_rules! impl_mixed {
    (
        $(
            $color_type:ty, $array_length:literal,
        )*
    ) => {
        $(
            impl<'a, 'c, U, T> Mixed<'a, U, T, $color_type, $array_length>
                for WithColormapImage<'a, 'c, U, T, $array_length>
            where
                T: PixelColor + Default + Invert + Screen + WeightedAvg,
                U: ImageDrawable<Color = $color_type> + Colors<$color_type>,
            {
                type Output<'b, 'd, V>
                    = WithColormapImageMix<'a, 'b, 'c, 'd, U, V, T, $array_length>
                where
                    T: 'd,
                    V: ImageDrawable<Color = $color_type> + Colors<$color_type> + 'b,
                    Self: 'a;

                fn mixed<'b, 'd, V>(
                    &'a self,
                    other: &'b Image<V>,
                    other_colormap: &'d Colormap<T, $array_length>,
                ) -> Self::Output<'b, 'd, V>
                where
                    V: ImageDrawable<Color = $color_type> + Colors<$color_type>,
                {
                    let area = self.image.bounding_box().intersection(&other.bounding_box());
                    let first = self.image.clipped(&area);
                    let second = other.clipped(&area);
                    let colormap = self.colormap;

                    WithColormapImageMix::new(first, second, colormap, other_colormap, area)
                }
            }
        )*
    }
}

impl_mixed! {
    BinaryColor, 2,
    Gray2, 4,
    Gray4, 16,
    Gray8, 256,
}
