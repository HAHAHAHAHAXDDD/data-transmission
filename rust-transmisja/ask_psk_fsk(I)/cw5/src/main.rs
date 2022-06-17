pub use iter_num_tools::lin_space;
extern crate iter_num_tools;
pub use euclid::*;
pub use std::f64::consts::PI;
pub use chfft::RFft1D;
pub use num_complex::Complex;
pub use tabled::{
    Tabled, Table, Style
};

#[derive(Tabled)]
struct BW{
    modulacja: &'static str,
    B3dB: f64,
    B6dB: f64,
    B12dB: f64,
}

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

pub fn mFFT(v1: Vec<f64>, N: f64, fs: f64) -> Vec<f64>{

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
    let result = Mp.clone();

    return result;
}

pub fn bandwidth(v1: Vec<f64>, db: f64) -> f64{
    let mut result = vec![];

    let maxVal = |b1: Vec<f64>| -> f64{
        let mut max:f64 = 0.;
        
        for i in b1{
            if i>max{
                max = i;
            }
        }
        return max;
    };

    let cop = v1.clone();
    let cop1 = v1.clone();

    let max = maxVal(cop);

    
    
    if db==3.{
        for i in v1{
            if i>max-3. && i<max{
                result.push(i);
            }
        }
    }
    else if db == 6.{
        for i in v1{
            if i>max-6. && i<max{
                result.push(i);
            }
        }
    }
    else if db == 12.{
        for i in v1{
            if i>max-12. && i<max{
                result.push(i);
            }
        }
    }
    let mut bw: f64 = 0.;
    if result.len()>=2{
        bw = result[result.len()-1]-result[0];
    }
    return f64::abs(bw);
}

fn main() {
    let b = ascii_to_bin("Elo");
    let bclone = b.clone();
    let B: f64 = bclone.len() as f64;
    let W: f64 = 2.;
    let fs: f64 = b.len() as f64 * 1000.; //czestotliwosc
    let tc: f64 = 1.; //czas symulacji
    let tb: f64 = tc/B;
    let _fn: f64 = W*(1./tb);
    let N: f64 = tc*fs;
    let a1: f64 = 1.;
    let a2: f64 = 2.;
    let i = lin_space(0.0..=N, N as usize);
    let mut t = vec![];
    for j in i{
        t.push(j/fs);
    }

    //ASK
    let b_ASK = b.clone();
    let t_ASK1 = t.clone();
    let r_ASK = ASK(t_ASK1, b_ASK, _fn, fs, a1, a2);

    //PSK
    let b_PSK = b.clone();
    let t_PSK1 = t.clone();
    let r_PSK = PSK(t_PSK1, b_PSK, _fn, fs);

    //FSK
    let _fn1 = (W+1.)/tb;
    let _fn2 = (W+2.)/tb;
    let b_FSK = b.clone();
    let t_FSK1 = t.clone();
    let r_FSK = FSK(t_FSK1, b_FSK, _fn1, _fn2, fs);


    //ASK
    let wASK = mFFT(r_ASK, N, fs);
    let wASK3 = wASK.clone();
    let bwASK3 = bandwidth(wASK3, 3.);
    let wASK6 = wASK.clone();
    let bwASK6 = bandwidth(wASK6, 6.);
    let wASK12 = wASK.clone();
    let bwASK12 = bandwidth(wASK12, 12.);

    //PSK
    let wPSK = mFFT(r_PSK, N, fs);
    let wPSK3 = wPSK.clone();
    let bwPSK3 = bandwidth(wPSK3, 3.);
    let wPSK6 = wPSK.clone();
    let bwPSK6 = bandwidth(wPSK6, 6.);
    let wPSK12 = wPSK.clone();
    let bwPSK12 = bandwidth(wPSK12, 12.);

    //FSK
    let wFSK = mFFT(r_FSK, N, fs);
    let wFSK3 = wFSK.clone();
    let bwFSK3 = bandwidth(wFSK3, 3.);
    let wFSK6 = wFSK.clone();
    let bwFSK6 = bandwidth(wFSK6, 6.);
    let wFSK12 = wFSK.clone();
    let bwFSK12 = bandwidth(wFSK12, 12.);

    
    let bandwidths = vec![
        BW{
            modulacja: "ASK",
            B3dB: bwASK3,
            B6dB: bwASK6,
            B12dB: bwASK12,
        },
        BW{
            modulacja: "PSK",
            B3dB: bwPSK3,
            B6dB: bwPSK6,
            B12dB: bwPSK12,
        },
        BW{
            modulacja: "FSK",
            B3dB: bwFSK3,
            B6dB: bwFSK6,
            B12dB: bwFSK12,
        },
    ];

    let table = Table::new(bandwidths).with(
        Style::modern()
    );

    println!("{}", table);
}
