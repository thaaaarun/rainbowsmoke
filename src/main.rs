mod render;
use render::render::save_image;

mod generate_ordering;
use generate_ordering::generate_ordering::GenerateOrdering;

mod color_selection;
use color_selection::color_selection::ColorSelectionModel;

mod create_image;
use create_image::create_image::CreateImage;

use std::time::Instant;

fn main() {
    let width = 1170; // x
    let height = 2532; // y
    let color_sample_size = 24000; // 1000x1000x5000 takes 7.5s, but with RTree, it only takes 3.5s
    let starting_points = vec![(0,width/2)];
    let input_image_path = "angryimg100100.png"; // this is currently 300x300
    let output_image_path = "output.png";
    let shuffle_neighbors = true;
    let random_variation = 0.85;
    let shuffle_input_image = true;

    // gotta serialize all these input parameters into 1 string
    // so i can name the output image file

    // initialize a color selection model
    let s1 = Instant::now();
    let color_selector =
        ColorSelectionModel::new(input_image_path, height, width, color_sample_size, shuffle_input_image);
    let d1 = s1.elapsed().as_secs_f64();
    println!("Color selection model took: {} seconds", d1);

    // generate an ordering
    let s2 = Instant::now();
    let gen_ordering = GenerateOrdering::new(
        height,
        width,
        starting_points,
        random_variation,
        shuffle_neighbors,
    );
    let d2 = s2.elapsed().as_secs_f64();
    println!("Generate ordering took: {} seconds", d2);

    // create an image!
    let s3 = Instant::now();
    let cr_image = CreateImage::new(color_selector, gen_ordering);
    let d3 = s3.elapsed().as_secs_f64();
    println!(
        "\x1b[31m{}\x1b[0m",
        format!("Create image took: {} seconds", d3)
    );
    let output_img = cr_image.output_img;

    // save the image with render
    let s4 = Instant::now();
    save_image(output_img, output_image_path);
    let d4 = s4.elapsed().as_secs_f64();
    println!("Render took: {} seconds", d4);

    // print total time
    let total_time = d1 + d2 + d3 + d4;
    println!("Total time: {} seconds", total_time);
    // print width, height, total no of pixels, and the sort length
    println!(
        "Width: {}, Height: {}, Total no of pixels: {}, Sort length: {}",
        height,
        width,
        width * height,
        color_sample_size
    );
}

// ALL COMPLETE AND WORKING!!!
// render                   [done] [working]
// color selection model    [done] [partially working]
// generate ordering        [done] [partially working]
// create image             [done] [partially working]
// looks like I have the same problem as in python with width and height getting mixed up somewhere :/
