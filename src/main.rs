// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]

use std::env::{args};
use std::process::{exit};

use image::{DynamicImage, ImageBuffer, open, Rgb};

use num_complex::{Complex};


fn main()
{
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = args().skip(1).collect();
    if args.is_empty()
    {
        print_usage_and_exit();
    }

    let subcommand = args.remove(0);
    match subcommand.as_str()
    {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" =>
        {
            if args.len() != 3 {print_usage_and_exit();}
            let infile      :String = args.remove(0);
            let outfile     :String = args.remove(0);
            let blurriness:f32    = args.remove(0).parse().expect("Couldn't parse blurriness.");

            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            blur(infile ,outfile ,blurriness);
        }

        // **OPTION**
        // Brighten -- see the brighten() function below
        "brighten" =>
        {
            if args.len() != 3 {print_usage_and_exit();}

            let infile     :String = args.remove(0);
            let outfile    :String = args.remove(0);
            let brightness :i32    = args.remove(0).parse().expect("Couldn't parse brightness.");
            brighten(infile ,outfile ,brightness);
       }

        // **OPTION**
        // Crop -- see the crop() function below
        "crop" =>
        {
            if args.len() != 6
            {
                print_usage_and_exit();
            }

            let infile  :String = args.remove(0);
            let outfile :String = args.remove(0);
            let startx  :u32    = args.remove(0).parse().expect("Couldn't parse value.");
            let starty  :u32    = args.remove(0).parse().expect("Couldn't parse value.");
            let width   :u32    = args.remove(0).parse().expect("Couldn't parse value.");
            let height  :u32    = args.remove(0).parse().expect("Couldn't parse value.");

            crop(infile ,outfile ,startx ,starty ,width ,height);
        }

        // **OPTION**
        // Rotate -- see the rotate() function below
        "rotate" =>
        {
            if args.len() != 3
            {
                print_usage_and_exit();
            }
            let infile   :String = args.remove(0);
            let outfile  :String = args.remove(0);
            let rotation :i32    = args.remove(0).parse().expect("Could not parse value");
            rotate(infile ,outfile ,rotation);
        }

        // **OPTION**
        // Invert -- see the invert() function below
        "invert_colors" =>
        {
            if args.len() != 2
            {
                print_usage_and_exit();
            }
            let infile   :String = args.remove(0);
            let outfile  :String = args.remove(0);
            invert_colors(infile ,outfile);
        }

        // **OPTION**
        // Grayscale -- see the grayscale() function below
        "grayscale" =>
        {
            if args.len() != 2
            {
                print_usage_and_exit();
            }
            let infile   :String = args.remove(0);
            let outfile  :String = args.remove(0);
            grayscale(infile ,outfile);
        }

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" =>
        {
            if args.len() != 1
            {
                print_usage_and_exit();
            }
            let outfile :String = args.remove(0);
            fractal(outfile);
        }

        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit()
{
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");

    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");

    exit(-1);
}

fn blur
(
    infile     :String
  , outfile    :String
  , blurriness :f32
)
{
    // Here's how you open an existing image file
    let img : DynamicImage = open(infile).expect("Failed to open INFILE.");

    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 :DynamicImage = img.blur(blurriness);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten
(   infile     :String // file to process
  , outfile    :String // file to which to save the result of processing
  , brightness :i32    // amount by which to brighten the image 
) 
{
    let img : DynamicImage = open(infile).expect("Failed to open INFILE.");

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    let img2 :DynamicImage = img.brighten(brightness);
    img2.save(outfile).expect("Could not write to output file.");
}

fn crop
(   infile  :String
  , outfile :String
  , startx  :u32
  , starty  :u32
  , width   :u32
  , height  :u32
)
{
    let mut img :DynamicImage = open(infile).expect("Failed to open INFILE.");

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.
    // Challenge: parse the four values from the command-line and pass them
    // through to this function.
    let img2 :DynamicImage = img.crop(startx ,starty ,width ,height);
    img2.save(outfile).expect("Could not write to output file.");
}

fn rotate
(
   infile   :String
  ,outfile  :String
  ,rotation :i32
)
{
    let img :DynamicImage = open(infile).expect("Failed to open INFILE.");

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.
    let img2 :DynamicImage =
    match rotation
    {
        90  => img.rotate90()
      , 180 => img.rotate180()
      , 270 => img.rotate270()
      , _   => Err("cannot rotate by specified degrees").expect("Error")
    };
    img2.save(outfile).expect("Could not write to output file.");
}

fn invert_colors
(
    infile  :String
  , outfile :String
)
{
    let mut img :DynamicImage = open(infile).expect("Failed to open INFILE.");

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    img.invert();
    img.save(outfile).expect("Could not write to output file.");
}

fn grayscale
(
   infile  :String
  ,outfile :String
)
{
    let img :DynamicImage = open(infile).expect("Failed to open INFILE.");

    // .grayscale() takes no arguments. It returns a new image.
    let img2 :DynamicImage = img.grayscale();
    img2.save(outfile).expect("Could not write to output file.");
}

fn generate(outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String)
{
    let width :u32 = 800;
    let height :u32 = 800;

    let mut ib :ImageBuffer<Rgb<u8> ,Vec<u8>> = ImageBuffer::new(width ,height);

    let scale_x :f32 = 3.0 / width as f32;
    let scale_y :f32 = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x,y,pixel) in ib.enumerate_pixels_mut()
    {
        // Use red and blue to be a pretty gradient background
        let red :u8 = (0.3 * x as f32) as u8;
        let blue:u8 = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx :f32 = y as f32 * scale_x - 1.5;
        let cy :f32 = x as f32 * scale_y - 1.5;

        let c = Complex::new(-0.45 ,0.6);
        let mut z = Complex::new(cx ,cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0
        {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = Rgb([red, green, blue]);
    }

    ib.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
