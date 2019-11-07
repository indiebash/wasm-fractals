use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{console, CanvasRenderingContext2d, ImageData};
mod utils;
extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const GRAPH_WIDTH: f64 = 4.0;
const GRAPH_HEIGHT: f64 = 3.0;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Clone, Copy, Debug)]
struct ComplexNumber {
    real: f64,
    imaginary: f64,
}

impl ComplexNumber {
    fn new(real: f64, imaginary: f64) -> ComplexNumber {
        ComplexNumber { real, imaginary }
    }

    fn square(self) -> ComplexNumber {
        let real = (self.real * self.real) - (self.imaginary * self.imaginary);
        let imaginary = 2.0 * self.real * self.imaginary;
        ComplexNumber { real, imaginary }
    }

    fn norm(&self) -> f64 {
        (self.real * self.real) + (self.imaginary * self.imaginary)
    }
}

impl Add<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl Add<f64> for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: f64) -> ComplexNumber {
        ComplexNumber {
            real: self.real + rhs,
            imaginary: self.imaginary,
        }
    }
}

#[wasm_bindgen]
pub fn generate_mandelbrot_image(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    iterations: u32,
    zoom: f64,
    offset_x: f64,
    offset_y: f64,
) -> Result<(), JsValue> {
    let mut data = generate_image(
        width,
        height,
        zoom,
        offset_x,
        offset_y,
        get_mandelbrot_stability(width, iterations),
    );
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

#[wasm_bindgen]
pub fn generate_julia_image(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
    iterations: u32,
    zoom: f64,
    offset_x: f64,
    offset_y: f64,
) -> Result<(), JsValue> {
    //log!("{}", "Generating Julia Image");
    let mut data = generate_image(
        width,
        height,
        zoom,
        offset_x,
        offset_y,
        get_julia_stability(width, iterations, ComplexNumber { real, imaginary }),
    );
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn generate_image<F>(
    width: u32,
    height: u32,
    zoom: f64,
    offset_x: f64,
    offset_y: f64,
    stability_function: F,
) -> Vec<u8>
where
    F: Fn(f64, f64) -> u32,
{
    let mut data = Vec::new();

    for x in 0..width {
        for y in 0..height {
            let param_r = 100.0 + offset_x + (zoom * 0.5);
            let param_i = -200.0 + offset_y + (zoom * 0.25);
            let scale = 1.0 + (zoom * 0.001);

            let stability =
                stability_function(x as f64 * scale - param_i, y as f64 * scale - param_r);

            data.push((stability / 4) as u8);
            data.push((stability / 2) as u8);
            data.push(stability as u8);
            data.push(255);
        }
    }

    data
}

fn get_mandelbrot_stability(canvas_width: u32, iterations: u32) -> Box<dyn Fn(f64, f64) -> u32> {
    return Box::new(move |x: f64, y: f64| {
        return get_point_stability(
            iterations,
            ComplexNumber::new(0.0, 0.0),
            coord_to_complex(x, y, canvas_width),
        );
    });
}

fn get_julia_stability(
    canvas_width: u32,
    iterations: u32,
    julia_constant: ComplexNumber,
) -> Box<dyn Fn(f64, f64) -> u32> {
    return Box::new(move |x: f64, y: f64| {
        return get_point_stability(
            iterations,
            coord_to_complex(x, y, canvas_width),
            julia_constant,
        );
    });
}

fn coord_to_complex(x: f64, y: f64, canvas_width: u32) -> ComplexNumber {
    let coords = utils::screen_coords_to_zero_centered_cartesian_coords(
        x,
        y,
        GRAPH_WIDTH,
        GRAPH_HEIGHT,
        canvas_width as f64,
    );
    return ComplexNumber::new(0.0, coords.1) + coords.0;
}

fn get_point_stability(iterations: u32, a: ComplexNumber, constant: ComplexNumber) -> u32 {
    let mut complex_number = a;
    let mut iteration_count: u32 = 0;
    while iteration_count < iterations {
        complex_number = complex_number.square() + constant;
        if complex_number.norm() > 2.0 {
            break;
        }
        iteration_count += 1;
    }
    iteration_count
}
