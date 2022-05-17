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
// use rand::distributions::{Normal, Distribution};

fn main() {
    //probki w dziedzinie czasu
    let f:f64 = 400.;
    let fs:f64 = 4000.; //czestotliwosc probkowania
    let fi:f64 = 2.*PI;
    let Tc:f64 = 2.; //czas trwania sygnalu
    let N:f64 = Tc*fs; //liczba probek przypadajacych na caly sygnal
    let n = lin_space(0.0..=N, N as usize);
    let mut t = vec![]; //czas odpowiadajacy kazdej probce n
    let mut buff = vec![];
    let x = |t:f64| -> f64{
        // (1.0-t)*(euclid::Trig::sin(2.0*PI*f*t*fi))*euclid::Trig::cos(4.0*PI*t)
        euclid::Trig::sin(2.0*PI*f*t)
    };
    for i in n{
        t.push(i/fs);
        buff.push(x(i/fs));
    }

    //dft
    let dft = |v1: Vec<f64>, N: i32| -> (Vec<f64>, Vec<f64>){
        let mut a = vec![0.; (N as usize)-1];
        let mut b = vec![0.; (N as usize)-1];
        for k in 0..N-1{
            for n in 0..N-1{
                a[k as usize] += v1[n as usize] * euclid::Trig::cos((-2.*PI*n as f64*k as f64)/N as f64);
                b[k as usize] += v1[n as usize] * euclid::Trig::sin((-2.*PI*n as f64*k as f64)/N as f64);
            }

        }
        return (a,b);
    };

    let copy = buff.clone();
    let s = copy.len();
    let (Rel, Im) = dft(buff, s as i32);
    let mut M = vec![];
    for i in 0..Rel.len()/2-1{
            M.push(f64::sqrt(f64::powf(Rel[i], 2.) + f64::powf(Im[i], 2.)));
        }
    let mut Mp = vec![];
    for i in 0..M.len(){
        Mp.push(10.*f64::log10(M[i]));
    }
    let mut fk = vec![];
    for i in 0..M.len(){
        fk.push(i as f64*fs/Rel.len() as f64);
    }
    let result = fk.into_iter().zip(Mp).collect::<Vec<_>>();
    let f1 = Plot::new(result).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let v = ContinuousView::new()
    .add(f1)
    // .x_range(-5., 50.)
    .y_range(-10., 50.);
    // .x_max_ticks(20);
    Page::single(&v).save("widmo.svg").expect("error");
}