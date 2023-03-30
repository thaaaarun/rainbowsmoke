pub mod generate_ordering {
    use rand::seq::SliceRandom;
    use rand::Rng;
    use std::collections::{HashMap, HashSet};

    pub struct GenerateOrdering {
        pub ordering: Vec<(usize, usize)>, // (x, y) coordinates
        pub parent_coords: HashMap<(usize, usize), (usize, usize)>, // key is child, value is parent

        // parameters
        pub width: usize,                     // width of the image
        pub height: usize,                    // height of the image
        starting_points: Vec<(usize, usize)>, // the starting points for the smoke
        random_variation: f32, // the amount of random variation in selecting neighbors from the stack
        shuffle_neighbors: bool, // whether to shuffle neighbors before selecting one
    }

    impl GenerateOrdering {
        pub fn new(
            width: usize,
            height: usize,
            starting_points: Vec<(usize, usize)>,
            random_variation: f32,
            shuffle_neighbors: bool,
        ) -> Self {
            let mut tmp = Self {
                ordering: Vec::new(),
                parent_coords: HashMap::new(),
                width,
                height,
                starting_points,
                random_variation,
                shuffle_neighbors,
            };
            tmp.generate_ordering_and_coords();
            tmp
        }

        fn get_neighbors(&self, pixel: (usize, usize)) -> Vec<(usize, usize)> {
            let mut neighbors: Vec<(usize, usize)> = Vec::new();
            let (x, y) = pixel;

            if x > 0 {
                neighbors.push((x - 1, y));
            }
            if x < self.width - 1 {
                neighbors.push((x + 1, y));
            }
            if y > 0 {
                neighbors.push((x, y - 1));
            }
            if y < self.height - 1 {
                neighbors.push((x, y + 1));
            }

            // Also Add the diagonal neighbors
            if x > 0 && y > 0 {
                neighbors.push((x - 1, y - 1));
            }
            if x < self.width - 1 && y > 0 {
                neighbors.push((x + 1, y - 1));
            }
            if x > 0 && y < self.height - 1 {
                neighbors.push((x - 1, y + 1));
            }
            if x < self.width - 1 && y < self.height - 1 {
                neighbors.push((x + 1, y + 1));
            }

            // shuffle_neighbors is true
            if self.shuffle_neighbors {
                let mut rng = rand::thread_rng();
                neighbors.shuffle(&mut rng);
            }

            neighbors
        }

        fn generate_ordering_and_coords(&mut self) {
            let mut visited_pixels: HashSet<(usize, usize)> = HashSet::new();
            let mut stack: Vec<(usize, usize)> = Vec::new();
            for pixel in &self.starting_points {
                self.parent_coords.insert(*pixel, *pixel); // A starting pixel is its own parent
                visited_pixels.insert(*pixel); // Mark pixel as visited
                self.ordering.push(*pixel); // Add pixel to ordering
                let mut neighbors = self.get_neighbors(*pixel); // Get neighbors of pixel
                neighbors.retain(|neighbor| !visited_pixels.contains(neighbor)); // Remove neighbors that have already been visited
                stack.extend(neighbors); // Add neighbors to stack
                for child in &stack {
                    self.parent_coords.insert(*child, *pixel); // Set parent of child to pixel
                }
            }

            // while the length of the stack is greater than 0
            while stack.len() > 0 {
                let current_pixel: (usize, usize);
                // if random_variation is 0, pop the last element of the stack
                match rand::random::<f32>() < self.random_variation {
                    true => {
                        let mut rng = rand::thread_rng();
                        let random_index = rng.gen_range(0..stack.len());
                        current_pixel = stack.swap_remove(random_index);
                    }
                    false => {
                        current_pixel = stack.pop().unwrap();
                    }
                }

                // if the current pixel is in visited, continue
                if visited_pixels.contains(&current_pixel) {
                    continue;
                }

                // add the current pixel to the ordering, and mark it as visited
                self.ordering.push(current_pixel);
                visited_pixels.insert(current_pixel);

                // get the neighbors of the current pixel
                let mut neighbors = self.get_neighbors(current_pixel);
                // remove neighbors that have already been visited
                neighbors.retain(|neighbor| !visited_pixels.contains(neighbor));
                //  if length of neighbors is 0, continue
                if neighbors.len() == 0 {
                    continue;
                }

                // set the parent of each neighbor child to the current pixel
                for neighbor in &neighbors {
                    self.parent_coords.insert(*neighbor, current_pixel);
                }
                // add neighbors to stack
                stack.extend(neighbors);
            } // end of while loop
        }
    }
}
