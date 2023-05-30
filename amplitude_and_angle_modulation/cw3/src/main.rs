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
    let mut fm:f64 = 1.;
    let mut _fn:f64 = 100.;
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
    
    let result1 = mFFT(buffa, N, fs);
    let result2 = mFFT(buffb, N, fs);
    let result3 = mFFT(buffc, N, fs);
    let result4 = mFFT(buffd, N, fs);
    let result5 = mFFT(buffe, N, fs);
    let result6 = mFFT(bufff, N, fs);
    let result7 = mFFT(buffg, N, fs);
    let result8 = mFFT(buffh, N, fs);
    let result9 = mFFT(buffi, N, fs);

    
    //Amplituda
    //(a)
    let rAaB3 = result1.clone();
    let aAB3 = bandwidth(rAaB3, 3.);
    let rAaB6 = result1.clone();
    let aAB6 = bandwidth(rAaB6, 6.);
    let rAaB12 = result1.clone();
    let aAB12 = bandwidth(rAaB12, 12.);
    //(b)
    let rAbB3 = result2.clone();
    let bAB3 = bandwidth(rAbB3, 3.);
    let rAbB6 = result2.clone();
    let bAB6 = bandwidth(rAbB6, 6.);
    let rAbB12 = result2.clone();
    let bAB12 = bandwidth(rAbB12, 12.);
    //(c)
    let rAcB3 = result3.clone();
    let cAB3 = bandwidth(rAcB3, 3.);
    let rAcB6 = result3.clone();
    let cAB6 = bandwidth(rAcB6, 6.);
    let rAcB12 = result3.clone();
    let cAB12 = bandwidth(rAcB12, 12.);

    //Faza
    //(a)
    let rFaB3 = result4.clone();
    let aFB3 = bandwidth(rFaB3, 3.);
    let rFaB6 = result4.clone();
    let aFB6 = bandwidth(rFaB6, 6.);
    let rFaB12 = result4.clone();
    let aFB12 = bandwidth(rFaB12, 12.);
    //(b)
    let rFbB3 = result5.clone();
    let bFB3 = bandwidth(rFbB3, 3.);
    let rFbB6 = result5.clone();
    let bFB6 = bandwidth(rFbB6, 6.);
    let rFbB12 = result5.clone();
    let bFB12 = bandwidth(rFbB12, 12.);
    //(c)
    let rFcB3 = result6.clone();
    let cFB3 = bandwidth(rFcB3, 3.);
    let rFcB6 = result6.clone();
    let cFB6 = bandwidth(rFcB6, 6.);
    let rFcB12 = result6.clone();
    let cFB12 = bandwidth(rFcB12, 12.);

    //Czestotliwosci
    //(a)
    let rCaB3 = result7.clone();
    let aCB3 = bandwidth(rCaB3, 3.);
    let rCaB6 = result7.clone();
    let aCB6 = bandwidth(rCaB6, 6.);
    let rCaB12 = result7.clone();
    let aCB12 = bandwidth(rCaB12, 12.);
    //(b)
    let rCbB3 = result8.clone();
    let bCB3 = bandwidth(rCbB3, 3.);
    let rCbB6 = result8.clone();
    let bCB6 = bandwidth(rCbB6, 6.);
    let rCbB12 = result8.clone();
    let bCB12 = bandwidth(rCbB12, 12.);
    //(c)
    let rCcB3 = result9.clone();
    let cCB3 = bandwidth(rCcB3, 3.);
    let rCcB6 = result9.clone();
    let cCB6 = bandwidth(rCcB6, 6.);
    let rCcB12 = result9.clone();
    let cCB12 = bandwidth(rCcB12, 12.);


    let bandwidths = vec![
        BW{
            modulacja: "Amplituda (a)",
            B3dB: aAB3,
            B6dB: aAB6,
            B12dB: aAB12,
        },
        BW{
            modulacja: "Amplituda (b)",
            B3dB: bAB3,
            B6dB: bAB6,
            B12dB: bAB12,
        },
        BW{
            modulacja: "Amplituda (c)",
            B3dB: cAB3,
            B6dB: cAB6,
            B12dB: cAB12,
        },
        BW{
            modulacja: "Faza (a)",
            B3dB: aFB3,
            B6dB: aFB6,
            B12dB: aFB12,
        },
        BW{
            modulacja: "Faza (b)",
            B3dB: bFB3,
            B6dB: bFB6,
            B12dB: bFB12,
        },
        BW{
            modulacja: "Faza (c)",
            B3dB: cFB3,
            B6dB: cFB6,
            B12dB: cFB12,
        },
        BW{
            modulacja: "Czestotliwosc (a)",
            B3dB: aCB3,
            B6dB: aCB6,
            B12dB: aCB12,
        },
        BW{
            modulacja: "Czestotliwosc (b)",
            B3dB: bCB3,
            B6dB: bCB6,
            B12dB: bCB12,
        },
        BW{
            modulacja: "Czestotliwosc (c)",
            B3dB: cCB3,
            B6dB: cCB6,
            B12dB: cCB12,
        },

    ];

    let table = Table::new(bandwidths).with(
        Style::modern()
    );

    println!("{}", table);
}
