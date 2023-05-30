pub use iter_num_tools::lin_space;
extern crate iter_num_tools;
pub use euclid::*;
pub use std::f64::consts::PI;
pub use realfft::RealFftPlanner;
pub use rustfft::{FftPlanner, num_complex::Complex};
pub use std::time::{Duration, Instant};
pub use std::convert::TryFrom;
pub use duration_string::*;
pub use tabled::{
    Tabled, Table, Style
};

#[derive(Tabled)]
struct Pomiar{
    funkcja: &'static str,
    ffttime: String,
    dfttime: String,
}

pub fn dft(v1: Vec<f64>, N: i32) -> (Vec<f64>, Vec<f64>){
    let mut a = vec![];
    let mut b = vec![];
    for k in 0..N-1{
        for n in 0..N-1{
            a.push(v1[n as usize] * euclid::Trig::cos((-2.*PI*n as f64*k as f64)/N as f64));
            b.push(v1[n as usize] * euclid::Trig::sin((-2.*PI*n as f64*k as f64)/N as f64));
        }
    }
    return (a,b);
}

fn main() {
    
    let f:f64 = 5.;
    let fs:f64 = 2000.; 
    let fsb:f64 = 4000.;
    let fi:f64 = 2.*PI;
    let Tc:f64 = 2.; 
    let Tcb: f64 = 2.;
    let N:f64 = Tc*fs; 
    let n = lin_space(0.0..=N, N as usize);
    let nb = lin_space(0.0..=(Tcb*fsb), (Tcb*fsb) as usize);
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
    
    let mut buffx = vec![];
    let mut buffy = vec![];
    let mut buffv = vec![];
    let mut buffz = vec![];
    let mut buffu = vec![];
    let mut buffb1 = vec![];
    let mut buffb2 = vec![];
    let mut buffb3 = vec![];
    
    let mut tu = vec![];
    for i in n{
        buffx.push(x(i/fs));
        buffy.push(y(i/fs));
        buffv.push(v(i/fs));
        buffz.push(z(i/fs));
        tu.push(i/fs);
    }
    let H = vec![5,20,60];
    let mut tb = vec![];
    for i in nb{
        tb.push(i/fs);
    }
    for i in tb{
        buffb1.push(b(i, H[0]));
        buffb2.push(b(i, H[1]));
        buffb3.push(b(i, H[2]));
    }
    for i in tu{
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
    //liczenie fft
    let mut planner = RealFftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(buffx.len());
    let mut result = fft.make_output_vec();
    let fftcalosc = Instant::now();
    let start = Instant::now();
    fft.process(&mut buffx, &mut result).unwrap();
    let durationx = start.elapsed();  
    let start = Instant::now();
    fft.process(&mut buffy, &mut result).unwrap();
    let durationy = start.elapsed();   
    let start = Instant::now();
    fft.process(&mut buffv, &mut result).unwrap();
    let durationv = start.elapsed();   
    let start = Instant::now();
    fft.process(&mut buffz, &mut result).unwrap();
    let durationz = start.elapsed();
    let start = Instant::now();
    fft.process(&mut buffu, &mut result).unwrap();
    let durationu = start.elapsed();
    let fft = planner.plan_fft_forward(buffb1.len());
    let mut result = fft.make_output_vec();
    let start = Instant::now();
    fft.process(&mut buffb1, &mut result).unwrap();
    let durationb1 = start.elapsed();  
    let start = Instant::now();
    fft.process(&mut buffb2, &mut result).unwrap();
    let durationb2 = start.elapsed();   
    let start = Instant::now();
    fft.process(&mut buffb3, &mut result).unwrap();
    let durationb3 = start.elapsed();
    let fftrcalosc = fftcalosc.elapsed();

    //liczenie dft
    let size1:i32 = buffx.len() as i32;
    let size2:i32 = buffb1.len() as i32;
    let scalosc = Instant::now();
    let start = Instant::now();
    let _ = dft(buffx, size1);
    let dftx = start.elapsed();
    let start = Instant::now();
    let _ = dft(buffy, size1);
    let dfty = start.elapsed();
    let start = Instant::now();
    let _ = dft(buffv, size1);
    let dftv = start.elapsed();
    let start = Instant::now();
    let _ = dft(buffz, size1);
    let dftz = start.elapsed();
    let start = Instant::now();
    let _ = dft(buffu, size1);
    let dftu = start.elapsed();
    let start = Instant::now();
    let _ = dft(buffb1, size2);
    let dftb1 = start.elapsed();
    let start = Instant::now();
    let _ = dft(buffb2, size2);
    let dftb2 = start.elapsed();
    let start = Instant::now();
    let _ = dft(buffb3, size2);
    let dftb3 = start.elapsed();
    let rcalosc = scalosc.elapsed();
    

    let pomiary = vec![
        Pomiar{
            funkcja: "X(t)",
            ffttime: DurationString::from(durationx).into(),
            dfttime: DurationString::from(dftx).into()
        },
        Pomiar{
            funkcja: "Y(t)",
            ffttime: DurationString::from(durationy).into(),
            dfttime: DurationString::from(dfty).into()
        },
        Pomiar{
            funkcja: "Z(t)",
            ffttime: DurationString::from(durationz).into(),
            dfttime: DurationString::from(dftz).into()
        },
        Pomiar{
            funkcja: "V(t)",
            ffttime: DurationString::from(durationv).into(),
            dfttime: DurationString::from(dftv).into()
        },
        Pomiar{
            funkcja: "U(t)",
            ffttime: DurationString::from(durationu).into(),
            dfttime: DurationString::from(dftu).into()
        },
        Pomiar{
            funkcja: "B1(t)",
            ffttime: DurationString::from(durationb1).into(),
            dfttime: DurationString::from(dftb1).into()
        },
        Pomiar{
            funkcja: "B2(t)",
            ffttime: DurationString::from(durationb2).into(),
            dfttime: DurationString::from(dftb2).into()
        },
        Pomiar{
            funkcja: "B3(t)",
            ffttime: DurationString::from(durationb3).into(),
            dfttime: DurationString::from(dftb3).into()
        },
        Pomiar{
            funkcja: "Wszystkie sygnaly",
            ffttime: DurationString::from(fftrcalosc).into(),
            dfttime: DurationString::from(rcalosc).into()
        },
    ];


    let table = Table::new(pomiary).with(
        Style::modern()
    );

    println!("{}", table);
}
