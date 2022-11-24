use std::{env, fs};
// use std::io::Write;


fn main() {
    // image header info
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    const RGB_MAX: i32 = 255;

    let mut img = String::new();

    let header: String = format!("P3\n {} {} \n {} \n", IMAGE_WIDTH, IMAGE_HEIGHT, RGB_MAX);
    img.push_str(&header);

    // render image left to right, up to down
    for y in (0..(IMAGE_WIDTH-1)).rev(){
        for x in 0..IMAGE_WIDTH {
            let r:f32 = x as f32 / (IMAGE_WIDTH -1) as f32;
            let g:f32 = y as f32 / (IMAGE_HEIGHT -1) as f32;
            let b:f32 = 0.25;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            // render based on RGB values
            let pixel: String = format!("{} {} {}\n", ir, ig, ib);
            img.push_str(&pixel);
        }
    }

    // println!("{}", img);
    // let mut new_img = fs::File::create("/tmp/image.ppm")
    //     .expect("can't create file");
    //
    // new_img.write_all(img.as_ref()).expect("can't convert String to reference");
    println!("{}", env::current_dir().expect("hello").display());

    fs::write("C:/Users/yaoke/CLionProjects/rust_raytracer/temp/image.ppm", img)
        .expect("Unable to write image data to file system");
}
