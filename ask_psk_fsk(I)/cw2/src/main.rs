pub use iter_num_tools::lin_space;
extern crate iter_num_tools;
pub use euclid::*;
pub use std::f64::consts::PI;
pub use plotters::prelude::*;


pub fn ascii_to_bin(s: &str) -> Vec<i32>{
    let mut binvec = vec![];
    for j in s.chars(){
        let ascii = j as i32;       
        for i in (0..=6).rev().step_by(1){
            let mut shiftval = ascii >> i;
            if ( shiftval & 1 ) != 0{
                binvec.push(1);
            }
            else{
                binvec.push(0);
            }
        }
    }
    return binvec;
}

pub fn draw_plot(v1: Vec<(f64, f64)>, fs: f64, name: String)  -> Result<(), Box<dyn std::error::Error>> {

    let mut filename = name.clone();
    let ext = ".svg";
    filename.push_str(ext);
    let root = SVGBackend::new(&filename, (1900, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let(upper, lower) = root.split_vertically(750);

    lower.titled(
        "wykresik",
        ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
    )?;
    
    let mut chart = ChartBuilder::on(&upper)
        .caption(name, ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (0f64..fs as f64),    
            (-3f64..3f64)
            // .log_scale()         
        )?;

        chart
            .configure_mesh()
            .draw()?;

        chart.draw_series(LineSeries::new(
                v1, &RED,
            ))?;

        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()?;
    Ok(())
}

pub fn ASK(t: Vec<f64>, b: Vec<i32>, _fn:f64, fs: f64, a1: f64, a2: f64) -> Vec<f64>{
    
    let za = |t:f64, _fn:f64, a: f64| -> f64{
        a*euclid::Trig::sin(2.*PI*_fn*t)
    };
    
    let mut result = vec![];
    let mut len = b.len() as i32;
    let mut start = 0;
    let mut stop = fs as i32/b.len() as i32;
    for i in b{
        for j in start..stop{
            if i == 0{
                result.push(za(t[j as usize], _fn, a1));
            }
            else if i == 1{
                result.push(za(t[j as usize], _fn, a2));
            }
        }
        start = stop;
        stop = stop + fs as i32/len;
    }
    return result;
}

pub fn PSK(t: Vec<f64>, b: Vec<i32>, _fn:f64, fs: f64) -> Vec<f64>{
    
    let zp_0 = |t:f64, _fn:f64| -> f64{
        euclid::Trig::sin(2.*PI*_fn*t)
    };
    
    let zp_1 = |t:f64, _fn:f64| -> f64{
        euclid::Trig::sin(2.*PI*_fn*t+PI)
    };
    
    let mut result = vec![];
    let mut len = b.len() as i32;
    let mut start = 0;
    let mut stop = fs as i32/b.len() as i32;
    for i in b{
        for j in start..stop{
            if i == 0{
                result.push(zp_0(t[j as usize], _fn));
            }
            else if i == 1{
                result.push(zp_1(t[j as usize], _fn));
            }
        }
        start = stop;
        stop = stop + fs as i32/len;
    }
    return result;
}

pub fn FSK(t: Vec<f64>, b: Vec<i32>, _fn1:f64, _fn2: f64, fs: f64) -> Vec<f64>{
    
    let zf = |t:f64, _fn:f64| -> f64{
        euclid::Trig::sin(2.*PI*_fn*t)
    };
    
    let mut result = vec![];
    let mut len = b.len() as i32;
    let mut start = 0;
    let mut stop = fs as i32/b.len() as i32;
    for i in b{
        for j in start..stop{
            if i == 0{
                result.push(zf(t[j as usize], _fn1));
            }
            else if i == 1{
                result.push(zf(t[j as usize], _fn2));
            }
        }
        start = stop;
        stop = stop + fs as i32/len;
    }
    return result;
}

fn main() {

    let b = ascii_to_bin("R");
    let n: f64 = b.len() as f64 - 1.;

    let bclone = b.clone();
    let B: f64 = bclone.len() as f64;

    let fs: f64 = b.len() as f64 * 1000.; //czestotliwosc
    let tc: f64 = 1.; //czas symulacji
    let N: f64 = tc*fs;
    let tb: f64 = tc/B;
    let _fn: f64 = n*(1./tb);
    let a1: f64 = 1.;
    let a2: f64 = 2.;
    let i = lin_space(0.0..=N, N as usize);
    let mut t = vec![];
    for j in i{
        t.push(j/fs);
    }
    //wyswietlenie bitow
    let x = b.clone();
    println!("{:?}", x);

    //ASK
    let b_ASK = b.clone();
    let t_ASK1 = t.clone();
    let t_ASK2 = t.clone();
    let r_ASK = ASK(t_ASK1, b_ASK, _fn, fs, a1, a2);
    let tup_ASK = t_ASK2.into_iter().zip(r_ASK).collect::<Vec<_>>();
    draw_plot(tup_ASK, tc, "ASK".to_string());

    //PSK
    let b_PSK = b.clone();
    let t_PSK1 = t.clone();
    let t_PSK2 = t.clone();
    let r_PSK = PSK(t_PSK1, b_PSK, _fn, fs);
    let tup_PSK = t_PSK2.into_iter().zip(r_PSK).collect::<Vec<_>>();
    draw_plot(tup_PSK, tc, "PSK".to_string());

    //FSK
    let _fn1 = (n+1.)/tb;
    let _fn2 = (n+2.)/tb;
    let b_FSK = b.clone();
    let t_FSK1 = t.clone();
    let t_FSK2 = t.clone();
    let r_FSK = FSK(t_FSK1, b_FSK, _fn1, _fn2, fs);
    let tup_FSK = t_FSK2.into_iter().zip(r_FSK).collect::<Vec<_>>();
    draw_plot(tup_FSK, tc, "FSK".to_string());
}