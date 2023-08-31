use rand::{self, Rng}; // 0.8.0
//use math::round::floor;
use image::{GenericImageView, ImageOutputFormat, Pixel};

fn main() {
    let mut rng = rand::thread_rng();
    let img_select = rng.gen_range(1..=12);

    println!("{}", img_select);

    let image = image::open("/home/oliver/Documents/imageToCmdln/src/393Piplup_DP_anime_5.png").unwrap();
    
    let new_image = image.resize_exact(100, 50, image::imageops::FilterType::Lanczos3);
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
            print!("{}", get_close_char(brightness));
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

fn get_close_char(brightness: f32) -> char{
    let mut selected_color: usize = (brightness*71.0).ceil() as usize;
    //println!("{}", selected_color);
    let grey_scale = " .'`^^,:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    if selected_color >= 70 {
        selected_color = 69;
    }
    let char = grey_scale.chars().nth(selected_color).unwrap();
    return char;
}