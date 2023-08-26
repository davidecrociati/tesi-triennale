use image::{DynamicImage, ImageFormat};

fn main() {
    // Open an image file
    let img: DynamicImage = image::open("images/3.jpg").expect("Failed to open image");

    // Resize the grayscale image to 400x600
    //let resized_img = img.resize_exact(400, 600, image::imageops::FilterType::Lanczos3);

    // Convert the image to grayscale
    let gray_img = img.grayscale();


    // Save the resized grayscale image
    gray_img.save_with_format("images/output_image.jpg", ImageFormat::Jpeg).expect("Failed to save image");
}
