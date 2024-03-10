use std::fs::File;
use std::ops::Deref;
use scrap::{Capturer, Display};
use image::{ImageOutputFormat, DynamicImage, ImageBuffer, Rgb};

pub fn capture_screenshot(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get the primary display
    let display = Display::primary()?;

    // Get the width and height of the display
    let (width, height) = (display.width() as u32, display.height() as u32);
    
    // Create a capturer for the display
    let mut capturer = Capturer::new(display)?;

    // Capture a screenshot
    let frame = capturer.frame()?;

    // Convert the frame into a dynamic image
    let image: DynamicImage = DynamicImage::ImageRgb8(ImageBuffer::from_fn(width, height, |x, y| {
        let index = 4 * (y * width + x) as usize;
        let pixel = &frame.deref()[index..index + 4];
        Rgb([pixel[0], pixel[1], pixel[2]])
    }));

    // Save the image to a file
    let mut file = File::create(file_path)?;
    image.write_to(&mut file, ImageOutputFormat::Png)?;

    Ok(())
}