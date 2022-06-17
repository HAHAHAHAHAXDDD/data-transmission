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
    let f:f64 = 5.;
    let fs:f64 = 8000.; //czestotliwosc probkowania
    let fi:f64 = 2.*PI;
    let Tc:f64 = 2.; //czas trwania sygnalu
    let N:f64 = Tc*fs; //liczba probek przypadajacych na caly sygnal
    let n = lin_space(0.0..=N, N as usize);
    let mut ty = vec![]; //czas odpowiadajacy kazdej probce n
    let mut tz = vec![];
    let mut tv = vec![];
    let mut buffy = vec![];
    let mut buffz = vec![];
    let mut buffv = vec![];
    let x = |t:f64| -> f64{
        (1.0-t)*(euclid::Trig::sin(2.0*PI*f*t*fi))*euclid::Trig::cos(4.0*PI*t)
    };
    let y = |t:f64| -> f64{
        f64::powf(-t,2.)*euclid::Trig::cos(t/0.2)*x(t)
    };
    let z = |t:f64| -> f64{
        x(t)*euclid::Trig::cos(2.*PI*f64::powf(t,2.)+PI)+0.276*f64::powf(t,2.)*x(t)
    };
    let v = |t:f64| -> f64{
        f64::sqrt(f64::abs(1.77-y(t)+z(t))*euclid::Trig::cos(5.2*PI*t)+x(t)+4.)
    };
    for i in n{
        ty.push(i/fs);
        tz.push(i/fs);
        tv.push(i/fs);
        buffy.push(y(i/fs));
        buffz.push(z(i/fs));
        buffv.push(v(i/fs));
    }
    let tupy = ty.into_iter().zip(buffy).collect::<Vec<_>>();
    let tupz = tz.into_iter().zip(buffz).collect::<Vec<_>>();
    let tupv = tv.into_iter().zip(buffv).collect::<Vec<_>>();
    let f1 = Plot::new(tupy).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let f2 = Plot::new(tupz).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round)            
    );
    let f3 = Plot::new(tupv).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round)            
    );
    let vy = ContinuousView::new().add(f1);
    let vz = ContinuousView::new().add(f2);
    let vv = ContinuousView::new().add(f3);
    Page::single(&vy).save("wykresY.svg").expect("error");
    Page::single(&vz).save("wykresZ.svg").expect("error");
    Page::single(&vv).save("wykresV.svg").expect("error");
}