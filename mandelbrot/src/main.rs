use num::Complex;

fn main() {
    println!("Hello, world!");
}

fn squre_loop(mut x: f64) {
    loop {
        x = x * x;
    }
}

fn squre_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

fn complex_squre_add_leep(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}
