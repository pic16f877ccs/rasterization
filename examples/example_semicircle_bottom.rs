use rasterization::{Rasterization, SemicircleFilled};
use image::{Rgb, RgbImage, ImageResult};

fn main() -> ImageResult<()> {
    let radius = 128_usize;
    let width = (radius * 2) as u32;
    let height = (radius * 2) as u32;
    let color = Rgb([217, 183, 114]);
    let center_x = radius as i32;
    let center_y = radius as i32;
    let mut img = RgbImage::from_pixel(width, height, Rgb([255, 255, 255]));
    SemicircleFilled::<i32>::new(radius)
        .semicircle_bottom()
        .offset(center_x, center_y) 
        .for_each(|(x, y)| { img.put_pixel(x as u32, y as u32, color); });
    
    img.save("semicircle_bottom.png")?;
    Ok(())
}

