use std::f64::consts::PI;
use turtle::Turtle;

use super::diacritics::with_title;

pub fn j(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size / 3.0);
    turtle.left(180.0);
    turtle.pen_down();
    for _ in 0..50 {
        turtle.forward(PI / 150.0 * font_size);
        turtle.left(3.6);
    }
    turtle.forward(font_size * 2.0 / 3.0);
    turtle.pen_up();
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size / 4.0);
    turtle.left(90.0);
}

pub fn k(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(45.0);
    turtle.pen_down();
    turtle.backward(2_f64.sqrt() * font_size / 2.0);
    turtle.right(90.0);
    turtle.forward(2_f64.sqrt() * font_size / 2.0);
    turtle.left(45.0);
    turtle.pen_up();
    turtle.forward(font_size / 4.0);
    turtle.left(90.0);
}

pub fn l(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.pen_up();
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size / 2.0);
    turtle.pen_up();
    turtle.forward(font_size / 4.0);
    turtle.left(90.0);
}

pub fn m(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.left(30.0);
    turtle.backward(1.25_f64.sqrt() * font_size);
    turtle.right(60.0);
    turtle.forward(1.25_f64.sqrt() * font_size);
    turtle.left(30.0);
    turtle.backward(font_size);
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(font_size / 4.0);
    turtle.left(90.0);
}

pub fn n(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.left(45.0);
    turtle.backward(2_f64.sqrt() * font_size);
    turtle.right(45.0);
    turtle.forward(font_size);
    turtle.pen_up();
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size / 4.0);
    turtle.left(90.0);
}

pub fn enie(turtle: &mut Turtle, font_size: f64) {
    with_title(turtle, font_size);
    n(turtle, font_size);
}

pub fn o(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(0.5 * font_size);
    turtle.pen_down();
    for _ in 0..100 {
        turtle.forward(PI / 100.0 * font_size);
        turtle.left(3.6);
    }
    turtle.pen_up();
    turtle.forward(font_size * 3.0 / 4.0);
    turtle.left(90.0);
}
