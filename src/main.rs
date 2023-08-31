use std::env;
use colored::Colorize;
use image::{GenericImageView, Pixel};


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_path = "";
    if args.len() > 1 {
        if args[1] == "--help"{
            println!("--image : image path");
            println!("--blocks : prints the image in blocks");
            println!("--ascii : prints the image in ascii");
            println!("--ascii_blocks : prints the image in both ascii and blocks");
            println!("default : ascii");
            return;
        } else if args[1] == "--image" {
            file_path = &args[2];
        }
    } else {
        println!("no args");
        return;
    }
    let image = image::open(file_path).unwrap();
    
    let new_image = image.resize_exact(60, 30, image::imageops::FilterType::Lanczos3);
    if let Err(err) = new_image.save("/home/oliver/Documents/imageToCmdln/src/temp.png"){
        println!("error");
        println!("{}", err);
    } else {
        println!("w");
    }
    
    for y in 0..new_image.dimensions().1 {
        for x in 0..new_image.dimensions().0 {
            let pix = new_image.get_pixel(x, y);
            let pix_color = pix.to_rgb();
            let brightness = get_brightness(pix_color[0], pix_color[1], pix_color[2]);
            if args.len() > 3{
                if args[2] == "--blocks" || args[3] == "--blocks" {
                    print!("{}", " ".on_truecolor(pix_color[0], pix_color[1], pix_color[2]));
                } else if args[2] == "--ascii" || args[3] == "--ascii" {
                    print!("{}", get_close_char(brightness).truecolor(pix_color[0], pix_color[1], pix_color[2]));
                } else if args[2] == "--ascii_blocks" || args[3] == "--ascii_blocks"{
                    print!("{}", get_close_char(brightness).on_truecolor(pix_color[0], pix_color[1], pix_color[2]));
                }
            } else {
                print!("{}", get_close_char(brightness).truecolor(pix_color[0], pix_color[1], pix_color[2]));    
            }
        }
        println!();
    }
}


fn get_brightness(red: u8, green: u8, blue: u8) -> f32{
    let r : f32 = red.into();
    let g : f32 = green.into();
    let b : f32 = blue.into();
    let brightness = (r/255.0) * 0.3 + (g/255.0) * 0.59 + (b/255.0) * 0.11;
    return brightness;
}

fn get_close_char(brightness: f32) -> String{
    let mut selected_color: usize = (brightness*71.0).ceil() as usize;
    let grey_scale = " .'`^^,:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    if selected_color >= 70 {
        selected_color = 69;
    }
    if selected_color < 5 {
        selected_color = 0
    }
    let char: String = grey_scale.chars().nth(selected_color).unwrap().into();
    return char;
}
