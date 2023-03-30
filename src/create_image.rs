pub mod create_image {
    use crate::color_selection::color_selection::ColorSelectionModel;
    use crate::generate_ordering::generate_ordering::GenerateOrdering;
    use std::collections::HashMap;

    pub struct CreateImage {
        // 1) a ColorSelectionModel, 2) an ordering, 3) a parent_coords, 4) an output_img
        color_selector: ColorSelectionModel,
        ordering: Vec<(usize, usize)>,
        parent_coords: HashMap<(usize, usize), (usize, usize)>,
        pub output_img: Vec<Vec<(u8, u8, u8)>>,
    }

    impl CreateImage {
        // Takes in a ColorSelectionModel and GenerateOrdering.
        pub fn new(
            color_selector: ColorSelectionModel,
            generate_ordering: GenerateOrdering,
        ) -> Self {
            let mut output_img: Vec<Vec<(u8, u8, u8)>> = Vec::new();
            for _ in 0..generate_ordering.width {
                let mut row: Vec<(u8, u8, u8)> = Vec::new();
                for _ in 0..generate_ordering.height {
                    row.push((0, 0, 0));
                }
                output_img.push(row);
            }

            let ordering = generate_ordering.ordering;
            let parent_coords = generate_ordering.parent_coords;
            let mut tmp = Self {
                color_selector,
                ordering,
                parent_coords,
                output_img,
            };
            tmp.generate_coloring();
            tmp
        }

        fn generate_coloring(&mut self) {
            // look at all the values of pixel, print the largest x coord and largest y coord
            let mut max_x = 0;
            let mut max_y = 0;
            for pixel in &self.ordering {
                if pixel.0 > max_x {
                    max_x = pixel.0;
                }
                if pixel.1 > max_y {
                    max_y = pixel.1;
                }
            }

            for curr_pixel in &self.ordering {
                if self.parent_coords[curr_pixel] == *curr_pixel {
                    // if a pixel is its own parent, then it was a starting pixel
                    self.output_img[curr_pixel.0][curr_pixel.1] =
                        self.color_selector.pop_random_color();
                } else {
                    // otherwise find the correct color from the parent pixel
                    let parent_pixel = self.parent_coords[curr_pixel];
                    let parent_color = self.output_img[parent_pixel.0][parent_pixel.1];

                    // @TODO: the error happens here, when assigning to output image.
                    self.output_img[curr_pixel.0][curr_pixel.1] =
                        self.color_selector.pop_nearest_neighbor(parent_color);
                }
            }
        }
    }
}
