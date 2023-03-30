pub mod render {
    use image::{ImageBuffer, Rgb};

    pub fn save_image(image_matrix: Vec<Vec<(u8, u8, u8)>>, filename: &str) {
        let width = image_matrix[0].len() as u32;
        let height = image_matrix.len() as u32;
        let mut img = ImageBuffer::new(width, height);

        for (x, row) in image_matrix.iter().enumerate() {
            for (y, (r, g, b)) in row.iter().enumerate() {
                img.put_pixel(y as u32, x as u32, Rgb([*r, *g, *b]));
            }
        }

        img.save(filename).unwrap();
    }

    // @TODO: extend module to render frames for a video or animation??
}
