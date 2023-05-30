pub use iter_num_tools::lin_space;
extern crate iter_num_tools;
pub use euclid::*;
pub use std::f64::consts::PI;
pub use plotlib::page::Page;
pub use plotlib::repr::Plot;
pub use plotlib::view::ContinuousView;
pub use plotlib::style::{LineJoin, LineStyle};

fn main() {
    
    let mut fm:f64 = 5.;
    let mut _fn:f64 = 50.;
    let Tc:f64 = 1.;
    let fs:f64 = 2.*_fn;
    let N:f64 = Tc*fs;
    let n = lin_space(0.0..=N, N as usize);
    let mut t = vec![];
    for i in n{
        t.push(i/fs);
    }
    // ton prosty na kiju
    let m = |t:f64| -> f64{
        euclid::Trig::sin(2.*PI*fm*t)
    };
    // Sygnal zmodulowany amplitudowo
    let za = |t:f64, ka:f64| -> f64{
        (ka*m(t)+1.)*euclid::Trig::cos(2.*PI*_fn*t)
    };
    // Sygnal zmodulowany kątowo: modulacja fazy
    let zp = |t:f64, kp:f64| -> f64{
        euclid::Trig::cos(2.*PI*_fn*t+kp*m(t))
    };
    //Sygnal zmodulowany kątowo: modulacja częstotliwości
    let zf = |t:f64, kf:f64| -> f64{
        euclid::Trig::cos(2.*PI*_fn*t+(kf/fm)*m(t))
    };

    let mut buffa = vec![];
    let mut buffb = vec![];
    let mut buffc = vec![];
    
    //modulacja amplitudy
    //a
    let rnzaa = t.clone();
    let tzaa = t.clone();
    for i in rnzaa{
        buffa.push(za(i, 0.2))
    }
    //b
    let tzab = t.clone();
    let rnzab = t.clone();
    for i in rnzab{
        buffb.push(za(i, 10.5))
    }
    //c
    let tzac = t.clone();
    let rnzac = t.clone();
    for i in rnzac{
        buffc.push(za(i, 50.))
    }

    //modulacja fazy
    let mut buffd = vec![];
    let mut buffe = vec![];
    let mut bufff = vec![];
    //a
    let rnzpa = t.clone();
    let tzpa = t.clone();
    for i in rnzpa{
        buffd.push(zp(i, 0.5))
    }
    //b
    let rnzpb = t.clone();
    let tzpb = t.clone();
    for i in rnzpb{
        buffe.push(zp(i, PI/2.))
    }
    //c
    let rnzpc = t.clone();
    let tzpc = t.clone();
    for i in rnzpc{
        bufff.push(zp(i, 2.*PI+1.))
    }

    //modulacja czestotliwosci
    let mut buffg = vec![];
    let mut buffh = vec![];
    let mut buffi = vec![];
    //a
    let rnzfa = t.clone();
    let tzfa = t.clone();
    for i in rnzfa{
        buffg.push(zf(i, 0.5))
    }
    //b
    let rnzfb = t.clone();
    let tzfb = t.clone();
    for i in rnzfb{
        buffh.push(zf(i, PI/2.))
    }
    //c
    let rnzfc = t.clone();
    let tzfc = t.clone();
    for i in rnzfc{
        buffi.push(zf(i, 3.*PI))
    }


    //ploty  
    //amplituda
    let tupzaa = tzaa.into_iter().zip(buffa).collect::<Vec<_>>();
    let tupzab = tzab.into_iter().zip(buffb).collect::<Vec<_>>();
    let tupzac = tzac.into_iter().zip(buffc).collect::<Vec<_>>();
    let f1 = Plot::new(tupzaa).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let f2 = Plot::new(tupzab).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let f3 = Plot::new(tupzac).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let v1 = ContinuousView::new().add(f1);
    let v2 = ContinuousView::new().add(f2);
    let v3 = ContinuousView::new().add(f3);
    Page::single(&v1).save("mAmplitudyA.svg").expect("error");
    Page::single(&v2).save("mAmplitudyB.svg").expect("error");
    Page::single(&v3).save("mAmplitudyC.svg").expect("error");
    
    //faza
    let tupzpa = tzpa.into_iter().zip(buffd).collect::<Vec<_>>();
    let tupzpb = tzpb.into_iter().zip(buffe).collect::<Vec<_>>();
    let tupzpc = tzpc.into_iter().zip(bufff).collect::<Vec<_>>();
    let f4 = Plot::new(tupzpa).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let f5 = Plot::new(tupzpb).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let f6 = Plot::new(tupzpc).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let v4 = ContinuousView::new().add(f4);
    let v5 = ContinuousView::new().add(f5);
    let v6 = ContinuousView::new().add(f6);
    Page::single(&v4).save("mFazyA.svg").expect("error");
    Page::single(&v5).save("mFazyB.svg").expect("error");
    Page::single(&v6).save("mFazyC.svg").expect("error");
    
    //czestotliwosc
    let tupzfa = tzfa.into_iter().zip(buffg).collect::<Vec<_>>();
    let tupzfb = tzfb.into_iter().zip(buffh).collect::<Vec<_>>();
    let tupzfc = tzfc.into_iter().zip(buffi).collect::<Vec<_>>();
    let f7 = Plot::new(tupzfa).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let f8 = Plot::new(tupzfb).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let f9 = Plot::new(tupzfc).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let v7 = ContinuousView::new().add(f7);
    let v8 = ContinuousView::new().add(f8);
    let v9 = ContinuousView::new().add(f9);
    Page::single(&v7).save("mCzestotliwosciA.svg").expect("error");
    Page::single(&v8).save("mCzestotliwosciB.svg").expect("error");
    Page::single(&v9).save("mCzestotliwosciC.svg").expect("error");
}
