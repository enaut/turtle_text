//! Draws text on the screen using the turtle.
//!
//! This example draws the text introduced as a first parameter
//! for example:
//!   cargo run --example letters 'Wow! So cool!'
//! Draw the text 'Wow! So cool!'
//! the optional second parameter is a font size, for example:
//!   cargo run --example letters 'Big text!' 50
//! Draw the text 'Big text!' with font size 50
//! if you don't insert any parameter:
//!   cargo run --example letters
//! this will draw a simple "hello, world!"

use std::env;
use std::process;
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();
    turtle.set_speed("fast");

    // Get the args
    let (text, font_size) = parse_args();

    // Go to the start of the text.
    go_to_initial_position(&mut turtle, &text, font_size);

    turtle_text::write_text(&mut turtle, &text, font_size);
    turtle.hide();
}

fn go_to_initial_position(turtle: &mut Turtle, text: &str, font_size: f64) {
    turtle.pen_up();
    turtle.left(90.0);
    turtle.forward(text.chars().count() as f64 * font_size / 2.0);
    turtle.right(90.0);
    turtle.pen_down();
}

/// Parses the command line arguments or exits with a help message if there was
/// an error
fn parse_args() -> (String, f64) {
    let mut args = env::args();

    // Skip the first argument (the executable name)
    args.next();

    // First argument is the text to draw
    let text = match args.next() {
        Some(text) if text == "--help" => print_help(),

        // This can produce any text, including the empty string
        Some(text) => text,
        // Default to `Hello, World!`
        None => "Hello, World!".to_string(),
    };

    // Second argument is the font size
    let font_size: f64 = match args.next() {
        Some(text) if text == "--help" => print_help(),

        Some(font_size) => match font_size.parse() {
            Ok(font_size) => {
                if font_size >= 1.0 {
                    font_size
                } else {
                    println!("Font size argument must be at least 1.0");
                    println!();
                    print_help();
                }
            }

            Err(err) => {
                println!("Font size argument must be a valid number: {}", err);
                println!();
                print_help();
            }
        },

        // Default to a font size of 20
        None => 20.0,
    };

    // Not expecting any other arguments
    if args.next().is_some() {
        print_help()
    }

    (text, font_size)
}

/// Prints the help message and then exits
///
/// `!` is the "never type" and it means this function never returns
fn print_help() -> ! {
    println!("Draws text on the screen using the turtle.");
    println!();
    println!("EXAMPLES:");
    println!("  cargo run --example letters 'Wow! So cool!'");
    println!("    Draw the text 'Wow! So cool!'");
    println!();
    println!("  cargo run --example letters 'Big text!' 50");
    println!("    Draw the text 'Big text!' with font size 50");
    println!();
    println!("  cargo run --example letters -- --help");
    println!("    Show this help information");
    println!();
    println!("FLAGS:");
    println!("  --help  show this help information");

    process::exit(0)
}
