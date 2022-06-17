pub use itertools::Itertools;
extern crate itertools;
pub use iter_num_tools::lin_space;
pub use iter_num_tools::arange;
extern crate iter_num_tools;
pub use euclid::*;
pub use std::f64::consts::PI;
pub use plotlib::page::Page;
pub use plotlib::repr::Plot;
pub use plotlib::view::ContinuousView;
pub use plotlib::style::{LineJoin, LineStyle};

fn main() {
    let dft = |v1: Vec<f64>, N: i32| -> (Vec<f64>, Vec<f64>){
        let mut a = vec![];
        let mut b = vec![];
        for k in 0..N-1{
            for n in 0..N-1{
                a.push(v1[n as usize] * euclid::Trig::cos((-2.*PI*n as f64*k as f64)/N as f64));
                b.push(v1[n as usize] * euclid::Trig::sin((-2.*PI*n as f64*k as f64)/N as f64));
            }
        }
        return (a,b);
    };
}
