pub use iter_num_tools::lin_space;
extern crate iter_num_tools;
pub use euclid::*;
pub use std::f64::consts::PI;
pub use chfft::RFft1D;
pub use num_complex::Complex;
pub use plotters::prelude::*;


pub fn mFFT(v1: Vec<f64>, N: f64, fs: f64) -> Vec<(f64,f64)>{

    let mut buff = v1.clone();
     
    let mut fft = RFft1D::<f64>::new(buff.len());
    let output = fft.forward(&buff);
    let mut Re = vec![];
    let mut Im = vec![];
    for i in 0..output.len(){
        Re.push(output[i].re);
        Im.push(output[i].im);
    }
    
    let mut M = vec![];
    for i in 0..(N/2.-1.) as usize{
            M.push(f64::sqrt(f64::powf(Re[i], 2.) + f64::powf(Im[i], 2.)));
        }
    let mut Mp = vec![];
    for i in 0..M.len(){
        Mp.push(10.*f64::log10(M[i]));
    }
    let mut fk = vec![];
    for i in 0..M.len(){
        fk.push(i as f64*(fs/N));
    }
    let result = fk.into_iter().zip(Mp).collect::<Vec<_>>();

    return result;
}

pub fn drawPlot(v1: Vec<(f64, f64)>, fs: f64, name: String)  -> Result<(), Box<dyn std::error::Error>> {

    let mut filename = name.clone();
    let ext = ".svg";
    filename.push_str(ext);
    let root = SVGBackend::new(&filename, (1900, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let(upper, lower) = root.split_vertically(750);

    lower.titled(
        "wykres widma funkcji lab3",
        ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
    )?;
    
    let mut chart = ChartBuilder::on(&upper)
        .caption(name, ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (50f64..fs as f64),           
            (-28f64..45f64)         
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

fn main(){

    let mut fm:f64 = 2.;
    let mut _fn:f64 = 200.;
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
        buffa.push(za(i, 0.5))
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
    

    let result1 = mFFT(buffa, N, fs);
    let result2 = mFFT(buffb, N, fs);
    let result3 = mFFT(buffc, N, fs);
    let result4 = mFFT(buffd, N, fs);
    let result5 = mFFT(buffe, N, fs);
    let result6 = mFFT(bufff, N, fs);
    let result7 = mFFT(buffg, N, fs);
    let result8 = mFFT(buffh, N, fs);
    let result9 = mFFT(buffi, N, fs);

    drawPlot(result1, fs, "AmplitudaA".to_string());
    drawPlot(result2, fs, "AmplitudaB".to_string());
    drawPlot(result3, fs, "AmplitudaC".to_string());
    drawPlot(result4, fs, "FazaA".to_string());
    drawPlot(result5, fs, "FazaB".to_string());
    drawPlot(result6, fs, "FazaC".to_string());
    drawPlot(result7, fs, "CzestotliwoscA".to_string());
    drawPlot(result8, fs, "CzestotliwoscB".to_string());
    drawPlot(result9, fs, "CzestotliwoscC".to_string());
}