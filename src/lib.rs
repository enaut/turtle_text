use turtle::Turtle;

// The implementations for each char works under the assumption that we will
// start the drawing in the left bottom corner on an imaginary rectangule
// heading to top.
// So each individual char impl will end in this default position for the next
// char to start.
mod letters;

pub fn write_text(turtle: &mut Turtle, text: &str, font_size: f64) {
    for character in text.chars() {
        // And for each char we check the corresponding implementation.
        use letters::*;
        match character {
            '0' => zero(turtle, font_size),
            '1' => one(turtle, font_size),
            '2' => two(turtle, font_size),
            '3' => three(turtle, font_size),
            '4' => four(turtle, font_size),
            '5' => five(turtle, font_size),
            '6' => six(turtle, font_size),
            '7' => seven(turtle, font_size),
            '8' => eight(turtle, font_size),
            '9' => nine(turtle, font_size),
            'a' | 'A' => a(turtle, font_size),
            'b' | 'B' => b(turtle, font_size),
            'c' | 'C' => c(turtle, font_size),
            'd' | 'D' => d(turtle, font_size),
            'e' | 'E' => e(turtle, font_size),
            'f' | 'F' => f(turtle, font_size),
            'g' | 'G' => g(turtle, font_size),
            'h' | 'H' => h(turtle, font_size),
            'i' | 'I' => i(turtle, font_size),
            'j' | 'J' => j(turtle, font_size),
            'k' | 'K' => k(turtle, font_size),
            'l' | 'L' => l(turtle, font_size),
            'm' | 'M' => m(turtle, font_size),
            'n' | 'N' => n(turtle, font_size),
            'ñ' | 'Ñ' => enie(turtle, font_size),
            'o' | 'O' => o(turtle, font_size),
            'p' | 'P' => p(turtle, font_size),
            'q' | 'Q' => q(turtle, font_size),
            'r' | 'R' => r(turtle, font_size),
            's' | 'S' => s(turtle, font_size),
            't' | 'T' => t(turtle, font_size),
            'u' | 'U' => u(turtle, font_size),
            'v' | 'V' => v(turtle, font_size),
            'w' | 'W' => w(turtle, font_size),
            'x' | 'X' => x(turtle, font_size),
            'y' | 'Y' => y(turtle, font_size),
            'z' | 'Z' => z(turtle, font_size),
            'á' | 'Á' | 'é' | 'É' | 'í' | 'Í' | 'ó' | 'Ó' | 'ú' | 'Ú' => {
                stressed_vowels(turtle, font_size, character)
            }
            '´' => acutte(turtle, font_size),
            '\'' => apostrophe(turtle, font_size),
            ':' => colon(turtle, font_size),
            ',' => comma(turtle, font_size),
            '.' => dot(turtle, font_size),
            '!' => exclamation(turtle, font_size),
            '¡' => inverted_exclamation(turtle, font_size),
            '¿' => inverted_question(turtle, font_size),
            '?' => question(turtle, font_size),
            ';' => semicolon(turtle, font_size),
            '~' => tilde(turtle, font_size),
            character if character.is_whitespace() => space(turtle, font_size),
            _ => {
                println!(
                    "We still don't have an implementation for the '{}' character!",
                    character
                );
                println!("but you can add it!:");
                println!("https://github.com/sunjay/turtle#contributing");
                question(turtle, font_size);
            }
        }
        turtle.pen_down();
    }
}

fn stressed_vowels(turtle: &mut Turtle, font_size: f64, character: char) {
    use letters::*;
    with_accute(turtle, font_size);
    match character {
        'á' | 'Á' => a(turtle, font_size),
        'é' | 'É' => e(turtle, font_size),
        'í' | 'Í' => i(turtle, font_size),
        'ó' | 'Ó' => o(turtle, font_size),
        _ => u(turtle, font_size),
    };
}
