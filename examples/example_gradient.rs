use rasterization::{Rasterization, SemicircleFilled};
use image::{Rgb, RgbImage, ImageResult};

fn main() -> ImageResult<()> {
    let radius = 128_usize;
    let width = (radius * 2) as u32;
    let height = (radius * 2) as u32;
    let center_x = radius as i32;
    let center_y = radius as i32;
    let mut img = RgbImage::from_pixel(width, height, Rgb([255, 255, 255]));
    SemicircleFilled::<i32>::new(radius)
        .circle()
        .offset(center_x, center_y) 
        .gradient(colorous::BROWN_GREEN, radius)
        .for_each(|(x, y, color)| { img.put_pixel(x as u32, y as u32, color.into()); });
    
    img.save("gradient.png")?;
    Ok(())
}
