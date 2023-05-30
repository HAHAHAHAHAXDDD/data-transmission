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
pub use plotters::prelude::*;
// use rand::distributions::{Normal, Distribution};

pub fn cdft (v1: Vec<f64>, N: i32, fs:f64) -> Vec<(f64, f64)>{
    let buff = v1.clone();
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
    return result;
}

pub fn makeplot(result:Vec<(f64, f64)>) -> plotlib::repr::Plot{
    let pl = Plot::new(result).line_style(
        LineStyle::new()
        .colour("burlywood")
        .linejoin(LineJoin::Round),      
    );
    return pl;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //probki w dziedzinie czasu
    let f:f64 = 2.;
    let fsx:f64 = 200.;
    let fsy:f64 = 200.;
    let fsv:f64 = 500.;
    let fsz:f64 = 200.;
    let fsu:f64 = 500.;
    let fsb:f64 = 4000.;
    let fi:f64 = 2.*PI;
    let Tc:f64 = 2.; //czas trwania sygnalu
    let nx = lin_space(0.0..=(Tc*fsx), (Tc*fsx) as usize);
    let ny = lin_space(0.0..=(Tc*fsy), (Tc*fsy) as usize);
    let nv = lin_space(0.0..=(Tc*fsv), (Tc*fsv) as usize);
    let nz = lin_space(0.0..=(Tc*fsz), (Tc*fsz) as usize);
    let nu = lin_space(0.0..=(Tc*fsu), (Tc*fsu) as usize);
    let nb1 = lin_space(0.0..=fsb, fsb as usize);
    let nb2 = lin_space(0.0..=fsb, fsb as usize);
    let nb3 = lin_space(0.0..=fsb, fsb as usize);
    let mut buffx = vec![];
    let mut buffy = vec![];
    let mut buffv = vec![];
    let mut buffz = vec![];
    let mut buffu = vec![];
    let mut buffb1 = vec![];
    let mut buffb2 = vec![];
    let mut buffb3 = vec![];
    let H = vec![5,20,60];
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
    let b = |t:f64, H:i32| -> f64{
        let mut sum:f64 = 0.;
        for h in 0..H {
            sum+=(1./(2.*(h as f64+1.)))*euclid::Trig::sin(((8.*h as f64 + 4.)*PI*t)+euclid::Trig::cos(6.*h as f64*PI*t))
        }
        return sum
    };
    for i in nx{
        buffx.push(x(i/fsx));
    }
    for i in ny{
        buffy.push(y(i/fsy));
    }
    for i in nz{
        buffz.push(z(i/fsz));
    }
    for i in nv{
        buffv.push(v(i/fsv));
    }
    let mut t = vec![];
    for i in nu{
        t.push(i/fsu);
    }
    for i in t{
        if i >= 0. && i < 0.5{
            buffu.push(u1(i));
        } else if i >= 0.5 && i < 1.9{
            buffu.push(u2(i));
        } else if i >= 1.9 && i < 3.7{
            buffu.push(u3(i));
        } else if i >= 3.7 && i < 4.9{
            buffu.push(u4(i));
        } else if i >= 4.9 && i < 6.4{
            buffu.push(u5(i));
        }
    }
    let mut tb = vec![];
    for i in nb1{
        tb.push(i/fsb);
    } 
    for i in tb{
        buffb1.push(b(i, H[0]));
        buffb2.push(b(i, H[1]));
        buffb3.push(b(i, H[2]));
    }
    let sx = buffx.len();
    let sy = buffy.len();
    let sv = buffv.len();
    let sz = buffz.len();
    let su = buffu.len();
    let sb1 = buffb1.len();
    let sb2 = buffb2.len();
    let sb3 = buffb3.len();
    let result1 = cdft(buffx, sx as i32, 250.);
    let result2 = cdft(buffy, sy as i32, 250.);
    let result3 = cdft(buffv, sv as i32, 500.);
    let result4 = cdft(buffz, sz as i32, 250.);
    let result5 = cdft(buffu, su as i32, 500.);
    let result6 = cdft(buffb1, sb1 as i32, 22050.);
    let result7 = cdft(buffb2, sb2 as i32, 22050.);
    let result8 = cdft(buffb3, sb3 as i32, 22050.);

    //wykresy: x,y,z
    let fx = makeplot(result1);
    let fy = makeplot(result2);
    let fz = makeplot(result4);

    let vx = ContinuousView::new().add(fx).y_range(-10., 20.).x_range(0., fsx/2.);
    let vy = ContinuousView::new().add(fy).y_range(-10., 20.).x_range(0., fsy/2.);
    let vz = ContinuousView::new().add(fz).y_range(-10., 20.).x_range(0., fsz/2.);

    Page::single(&vx).save("widmoX.svg").expect("error");
    Page::single(&vy).save("widmoY.svg").expect("error");
    Page::single(&vz).save("widmoZ.svg").expect("error");
     
    //plotters (dla bardziej skomplikowanych? wykresow)
    //wykresy: v, u, b1, b2, b3
    let root = SVGBackend::new("widmoV.svg", (1900, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let(upper, lower) = root.split_vertically(750);

    lower.titled(
        "wykres widma funkcji v z lab1",
        ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
    )?;
    
    let mut chart = ChartBuilder::on(&upper)
        .caption("widmo V", ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (0.45f64..fsv+50 as f64)
                .log_scale(),
            (-10f64..40f64)
        )?;

        chart
            .configure_mesh()
            .draw()?;

        chart.draw_series(LineSeries::new(
                result3, &RED,
            ))?;

        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()?;
    

    let root = SVGBackend::new("widmoU.svg", (1900, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let(upper, lower) = root.split_vertically(750);

    lower.titled(
        "wykres widma funkcji u z lab1",
        ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
    )?;

    let mut chart = ChartBuilder::on(&upper)
        .caption("widmo U", ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (0f64..fsu/2 as f64),
            (-20f64..50f64)
        )?;

        chart
            .configure_mesh()
            .draw()?;

        chart.draw_series(LineSeries::new(
                result5, &RED,
            ))?;

        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()?;

            
    let root = SVGBackend::new("widmoB1.svg", (1900, 768)).into_drawing_area();
        root.fill(&WHITE)?;
        
        let(upper, lower) = root.split_vertically(750);
        
        lower.titled(
            "wykres widma funkcji b1 z lab1",
            ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
        )?;
        
    let mut chart = ChartBuilder::on(&upper)
        .caption("widmo B1", ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (6f64..fsb/2 as f64)
                .log_scale(),
            (-10f64..50f64)
                // .log_scale(),
        )?;
        
        chart
            .configure_mesh()
            .draw()?;
        
        chart.draw_series(LineSeries::new(
                result6, &RED,
            ))?;
        
        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()?;
    let root = SVGBackend::new("widmoB2.svg", (1900, 768)).into_drawing_area();
         root.fill(&WHITE)?;
                
        let(upper, lower) = root.split_vertically(750);
                
        lower.titled(
            "wykres widma funkcji b2 z lab1",
            ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
        )?;
                
    let mut chart = ChartBuilder::on(&upper)
        .caption("widmo B2", ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (6f64..fsb/2 as f64)
            .log_scale(),
            (-10f64..50f64)
            // .log_scale(),
        )?;
                
            chart
                .configure_mesh()
                .draw()?;
                
            chart.draw_series(LineSeries::new(
                 result7, &RED,
            ))?;
                
            chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()?;
    let root = SVGBackend::new("widmoB3.svg", (1900, 768)).into_drawing_area();
        root.fill(&WHITE)?;
                        
        let(upper, lower) = root.split_vertically(750);
                        
            lower.titled(
                "wykres widma funkcji b3 z lab1",
                ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
                )?;
                        
    let mut chart = ChartBuilder::on(&upper)
            .caption("widmo B3", ("sans-serif", (5).percent_height()))
            .set_label_area_size(LabelAreaPosition::Left, (8).percent())
            .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
            .margin((1).percent())
            .build_cartesian_2d(
                (6f64..fsb/2 as f64),
                (-10f64..50f64)
            )?;
                        
        chart
            .configure_mesh()
            .draw()?;
                        
        chart.draw_series(LineSeries::new(
            result8, &RED,
            ))?;
                        
        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()?;

      
        
    Ok(())
}