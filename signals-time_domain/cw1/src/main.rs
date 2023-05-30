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
//Funkcja z tabeli: 8
fn main(){
    let f:f64 = 5.;
    let fs:f64 = 8000.; //czestotliwosc probkowania
    let fi:f64 = 2.*PI;
    let Tc:f64 = 2.; //czas trwania sygnalu
    let N:f64 = Tc*fs; //liczba probek przypadajacych na caly sygnal
    let n = lin_space(0.0..=N, N as usize);
    let mut t = vec![]; //czas odpowiadajacy kazdej probce n
    let mut buff = vec![];
    let x = |t:f64| -> f64{
        (1.0-t)*(euclid::Trig::sin(2.0*PI*f*t*fi))*euclid::Trig::cos(4.0*PI*t)
    };
    for i in n{
        t.push(i/fs);
        buff.push(x(i/fs));
    }
    let tup = t.into_iter().zip(buff).collect::<Vec<_>>();
    let f1 = Plot::new(tup).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let v = ContinuousView::new().add(f1);
    Page::single(&v).save("wykresX.svg").expect("error");
}