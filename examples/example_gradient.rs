use image::{ImageResult, Rgb, RgbImage};
use rasterization::{DirectionGradient, Rasterization, SemicircleFilled};

fn main() -> ImageResult<()> {
    use crate::DirectionGradient::*;
    let radius = 256_usize;
    let width = (radius / 4 * 9) as u32;
    let height = (radius / 4 * 9) as u32;
    let center_x = (width / 2) as i32;
    let center_y = (height / 2) as i32;
    let offset = (radius as f32 * 1.414).ceil() as i32;
    let size = offset as usize;
    let grad = colorous::CUBEHELIX;
    let mut img = RgbImage::from_pixel(width, height, Rgb([255, 255, 255]));
    let iter = SemicircleFilled::<i32>::new(radius);
    iter.clone()
        .semicircle_top()
        .gradient(offset, size, BottomRight(grad))
        .chain(
            iter.clone()
                .third_quadrant(0)
                .gradient(offset, size, TopRight(grad)),
        )
        .chain(
            iter.fourth_quadrant(0)
                .gradient(offset, size, TopRight(grad)),
        )
        .for_each(|(x, y, color)| {
            img.put_pixel((x + center_x) as u32, (y + center_y) as u32, color.into());
        });

    img.save("gradient.png")?;
    Ok(())
}
