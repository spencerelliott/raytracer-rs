use std::fs::File;
use std::io::prelude::*;

pub trait Exporter {
    fn push_pixel(&mut self, r: f32, g: f32, b: f32);
    fn finish(&mut self);
}

pub struct OidnExporter {
    pixels: Vec<f32>,
    cur_idx: usize,
    file: String,
    width: usize,
    height: usize,
}

impl OidnExporter {
    pub fn new(width: i32, height: i32, file: String) -> OidnExporter {
        OidnExporter {
            pixels: vec![],
            cur_idx: 0,
            file: file,
            width: width as usize,
            height: height as usize,
        }
    }

    fn add_item(pixels: &mut Vec<f32>, value: f32) {
        pixels.push(value);
    }
}

impl Exporter for OidnExporter {
    fn push_pixel(&mut self, r: f32, g: f32, b: f32) {
        Self::add_item(&mut self.pixels, r / 255.99);
        Self::add_item(&mut self.pixels, g / 255.99);
        Self::add_item(&mut self.pixels, b / 255.99);
    }

    fn finish(&mut self) {
        let device = oidn::Device::new();
        let mut filter = oidn::RayTracing::new(&device);
        filter.set_srgb(true).set_img_dims(self.width, self.height);
        let mut output = vec![0.0f32; 3 * (self.width * self.height)];

        match filter.execute(&self.pixels[..], &mut output[..]) {
            Err(_) => {}
            _ => {}
        }

        if let Err(error) = device.get_error() {
            panic!("Denoising error: {}", error.1);
        }

        let mut output_img = vec![0u8; output.len()];
        for i in 0..output.len() {
            let p = output[i] * 255.0;
            if p < 0.0 {
                output_img[i] = 0;
            } else if p > 255.0 {
                output_img[i] = 255;
            } else {
                output_img[i] = p as u8;
            }
        }

        if let Err(_) = image::save_buffer("out.jpg", &output_img, self.width as u32, self.height as u32, image::RGB(8)) {
            panic!("Error in image output");
        }
    }
}

pub struct P3Exporter {
    p3_file: File,
}

impl P3Exporter {
    pub fn new(width: i32, height: i32, file: String) -> P3Exporter {
        if let Ok(mut p3_file) = File::create(file) {
            match p3_file.write_fmt(format_args!("P3\n{} {}\n255\n", width, height)) {
                Ok(_) => return P3Exporter { p3_file: p3_file },
                _ => panic!("Could not write format"),
            }
        }

        panic!("Could not create file");
    }
}

impl Exporter for P3Exporter {
    fn push_pixel(&mut self, r: f32, g: f32, b: f32) {
        let mut mutable_file = &self.p3_file;
        match mutable_file.write_fmt(format_args!("{} {} {}\n", r as i32, g as i32, b as i32)) {
            Ok(_) => {}
            _ => {}
        }
    }

    fn finish(&mut self) {
        let mut mutable_file = &self.p3_file;
        match mutable_file.flush() {
            Ok(_) => {}
            _ => {}
        }
    }
}
