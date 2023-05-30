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
//Funkcja z tabeli: 4
fn main(){
    let fs:f64 = 8000.; //czestotliwosc probkowania
    let Tc:f64 = 2.; //czas trwania sygnalu
    let N:f64 = Tc*fs; //liczba probek przypadajacych na caly sygnal
    let n = lin_space(0.0..=N, N as usize);
    let mut t = vec![]; //czas odpowiadajacy kazdej probce n
    let mut buff = vec![];
    let u1 = |t:f64| -> f64{
        0.9*euclid::Trig::sin(2.*PI*t*8.-(PI/3.))+f64::log2(f64::abs(euclid::Trig::cos(7.*f64::powf(t,2.))+2.2))
    };
    let u2 = |t:f64| -> f64{
        (euclid::Trig::sin(2.*euclid::Trig::cos(4.*PI*t)*PI*t))/(2.*f64::powf(t,2.)+1.)
    };
    let u3 = |t:f64| -> f64{
        f64::powf(t-1.9, 2.)-euclid::Trig::cos(13.*t)
    };
    let u4 = |t:f64| ->  f64{
        0.5*f64::powf(t, 0.7)*euclid::Trig::sin(8.*t)
    };
    let u5 = |t:f64| -> f64{
        (2.+euclid::Trig::sin(18.*t))/(3.+euclid::Trig::cos(28.*t))
    };
    for i in n{
        t.push(i/fs);
    }
    let tcopy = t.clone();
    for i in tcopy{
        if i >= 0. && i < 0.5{
            buff.push(u1(i));
        } else if i >= 0.5 && i < 1.9{
            buff.push(u2(i));
        } else if i >= 1.9 && i < 3.7{
            buff.push(u3(i));
        } else if i >= 3.7 && i < 4.9{
            buff.push(u4(i));
        } else if i >= 4.9 && i < 6.4{
            buff.push(u5(i));
        }
    }
    let tup = t.into_iter().zip(buff).collect::<Vec<_>>();
    let f1 = Plot::new(tup).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let v = ContinuousView::new().add(f1);
    Page::single(&v).save("wykresU.svg").expect("error");
}