use image::{DynamicImage, ImageFormat};

fn main() {
    // Open an image file
    let img: DynamicImage = image::open("3.jpg").expect("Failed to open image");

    // Convert the image to grayscale
    let gray_img = img.grayscale();

    // Resize the grayscale image to 400x600
    let resized_img = gray_img.resize_exact(400, 600, image::imageops::FilterType::Lanczos3);

    // Save the resized grayscale image
    resized_img.save_with_format("output_image.jpg", ImageFormat::Jpeg).expect("Failed to save image");
}
