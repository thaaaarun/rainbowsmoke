pub mod color_selection {
    use image;
    use rand::Rng;
    use rstar::RTree; // Using RTree is worse for small sizes, but much faster for large sizes

    pub struct ColorSelectionModel {
        remaining_colors: Vec<(u8, u8, u8)>,
        tree: RTree<(i32, i32, i32)>,
    }

    impl ColorSelectionModel {
        pub fn new(
            image_path: &str,
            width: usize,
            height: usize,
            sort_length: usize,
            randomize_image: bool,
        ) -> Self {
            let mut remaining_colors: Vec<(u8, u8, u8)> = Vec::new();
            let mut tree: RTree<(i32, i32, i32)> = RTree::new();

            // add the first sort_length colors to colors_to_search, and save the rest to remaining_colors
            let mut idx = 0;
            let img = image::open(image_path).expect("File not found!");
            let mut img = image::imageops::resize(
                &img,
                width as u32,
                height as u32,
                image::imageops::FilterType::Nearest,
            );

            // we can randomly swap the order of colors in the image
            // iterate through width and height and randomly swap the color at that index with another
            if randomize_image {
                let mut rng = rand::thread_rng();
                for i in 0..img.width() {
                    for j in 0..img.height() {
                        let random_i = rng.gen_range(0..img.width());
                        let random_j = rng.gen_range(0..img.height());
                        // now we need to set the different colors
                        let curr_r = img.get_pixel(i, j).0[0];
                        let curr_g = img.get_pixel(i, j).0[1];
                        let curr_b = img.get_pixel(i, j).0[2];

                        let random_r = img.get_pixel(random_i, random_j).0[0];
                        let random_g = img.get_pixel(random_i, random_j).0[1];
                        let random_b = img.get_pixel(random_i, random_j).0[2];

                        img.put_pixel(i, j, image::Rgba([random_r, random_g, random_b, 255]));
                        img.put_pixel(
                            random_i,
                            random_j,
                            image::Rgba([curr_r, curr_g, curr_b, 255]),
                        );
                    }
                }
            }

            for pixel in img.pixels() {
                let r = pixel.0[0];
                let g = pixel.0[1];
                let b = pixel.0[2];

                if idx < sort_length {
                    tree.insert((r as i32, g as i32, b as i32));
                } else {
                    remaining_colors.push((r, g, b));
                }

                idx += 1;
            }

            // reverse remaining_colors so we can pop off easily later
            remaining_colors.reverse();

            Self {
                remaining_colors,
                tree,
            }
        }

        fn maintain_tree_size(&mut self) {
            if self.remaining_colors.len() > 0 {
                let color_to_push = self.remaining_colors.pop().unwrap();
                self.tree.insert((
                    color_to_push.0 as i32,
                    color_to_push.1 as i32,
                    color_to_push.2 as i32,
                ));
            }
        }

        pub fn pop_random_color(&mut self) -> (u8, u8, u8) {
            let mut rng = rand::thread_rng();
            let random_color = (
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            );
            let nearest = self.get_nearest_neighbor_from_tree(random_color);
            self.tree.remove(&nearest);
            self.maintain_tree_size();
            (nearest.0 as u8, nearest.1 as u8, nearest.2 as u8)
        }

        pub fn pop_nearest_neighbor(&mut self, query_color: (u8, u8, u8)) -> (u8, u8, u8) {
            let nearest = self.get_nearest_neighbor_from_tree(query_color);
            self.tree.remove(&nearest);
            self.maintain_tree_size();
            (nearest.0 as u8, nearest.1 as u8, nearest.2 as u8)
        }

        fn get_nearest_neighbor_from_tree(&mut self, query_color: (u8, u8, u8)) -> (i32, i32, i32) {
            let nearest = self
                .tree
                .nearest_neighbor(&(
                    query_color.0 as i32,
                    query_color.1 as i32,
                    query_color.2 as i32,
                ))
                .unwrap();

            (nearest.0, nearest.1, nearest.2)
        }
    }
}
