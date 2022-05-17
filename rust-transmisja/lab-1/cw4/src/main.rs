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
    let fs:f64 = 22050.; //czestotliwosc probkowania
    let Tc:f64 = 1.; //czas trwania sygnalu
    let N:f64 = Tc*fs; //liczba probek przypadajacych na caly sygnal
    let n = lin_space(0.0..=N, N as usize);
    let mut t = vec![]; //czas odpowiadajacy kazdej probce n
    let mut buffb1 = vec![];
    let mut buffb2 = vec![];
    let mut buffb3 = vec![];
    let H = vec![5,20,60];
    for i in n{
        t.push(i/fs);
    } 
    let t1 = t.clone();
    let t2 = t.clone();
    let t3 = t.clone();
    let b = |t:f64, H:i32| -> f64{
        let mut sum:f64 = 0.;
        for h in 0..H {
            sum+=(1./(2.*(h as f64+1.)))*euclid::Trig::sin(((8.*h as f64 + 4.)*PI*t)+euclid::Trig::cos(6.*h as f64*PI*t))
        }
        return sum
    };
    for i in t{
        buffb1.push(b(i, H[0]));
        buffb2.push(b(i, H[1]));
        buffb3.push(b(i, H[2]));
    }
    let tupb1 = t1.into_iter().zip(buffb1).collect::<Vec<_>>();
    let tupb2 = t2.into_iter().zip(buffb2).collect::<Vec<_>>();
    let tupb3 = t3.into_iter().zip(buffb3).collect::<Vec<_>>();
    let f1 = Plot::new(tupb1).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let f2 = Plot::new(tupb2).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let f3 = Plot::new(tupb3).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let vb1 = ContinuousView::new().add(f1);
    let vb2 = ContinuousView::new().add(f2);
    let vb3 = ContinuousView::new().add(f3);
    Page::single(&vb1).save("wykresB1.svg").expect("error");
    Page::single(&vb2).save("wykresB2.svg").expect("error");
    Page::single(&vb3).save("wykresB3.svg").expect("error");
}