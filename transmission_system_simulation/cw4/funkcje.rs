#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_parens)]
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

pub fn dASK(ask: Vec<f64>,  t: Vec<f64>, _fn: f64, a: f64, fs: f64, tb: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>){
    let mut resultx = vec![];
    let x = |t:f64, _fn:f64, a: f64| -> f64{
        a*euclid::Trig::sin(2.*PI*_fn*t)
    };
    for i in 0..(ask.len()){
        resultx.push(ask[i]*x(t[i], _fn, a));
    }
    let mut resultp = vec![];
    let wrx = resultx.clone();
    let mut stop = tb;
    let mut sum = 0.;
    for i in 0..(wrx.len()){
        sum = sum + wrx[i];
        resultp.push(sum);
        if  t[i]>stop{
            sum = 0.;
            resultp[i] = sum;
            stop = stop+tb;
        }
    }
    let mut wrp = resultp.clone();
    let h = 350.;
    let mut resultc = vec![];
    let interval = (fs*tb)-1.;
    let mut stop = interval;
    for i in 0..wrp.len(){
        if i == stop as usize{
            if wrp[i] > h{
                for i in resultc.len()..(stop+1.) as usize{
                    resultc.push(1.);
                }
            }
            else{
                for i in resultc.len()..(stop+1.) as usize{
                    resultc.push(0.);
                }
            }
            stop = stop + interval;
        }
    }
    return (resultx, resultp, resultc);
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

pub fn dPSK(psk: Vec<f64>,  t: Vec<f64>, _fn: f64, a: f64, fs: f64, tb: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>){
    let mut resultx = vec![];
    let x = |t:f64, _fn:f64, a: f64| -> f64{
        a*euclid::Trig::sin(2.*PI*_fn*t)
    };
    for i in 0..(psk.len()){
        resultx.push(psk[i]*x(t[i], _fn, a));
    }
    let mut resultp = vec![];
    let wrx = resultx.clone();
    let mut stop = tb;
    let mut sum = 0.;
    for i in 0..(wrx.len()){
        sum = sum + wrx[i];
        resultp.push(sum);
        if  t[i]>stop{
            sum = 0.;
            resultp[i] = sum;
            stop = stop+tb;
        }
    }
    let mut wrp = resultp.clone();
    let mut resultc = vec![];
    for i in wrp{
        if i < 0.{
            resultc.push(1.);
        }
        else{
            if resultc.len() > 0 && i == 0. && resultc[resultc.len()-1] == 1.{
                resultc.push(1.);
            }
            else{
                resultc.push(0.);
            }
        }
    }
    return (resultx, resultp, resultc);
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

pub fn dFSK(fsk: Vec<f64>,  t: Vec<f64>, _fn1: f64, _fn2: f64, a: f64, fs: f64, tb: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>){
    let mut resultx1 = vec![];
    let mut resultx2 = vec![];
    let x = |t:f64, _fn:f64, a: f64| -> f64{
        a*euclid::Trig::sin(2.*PI*_fn*t)
    };
    for i in 0..(fsk.len()){
        resultx1.push(fsk[i]*x(t[i], _fn1, a));
        resultx2.push(fsk[i]*x(t[i], _fn2, a));
    }
    let mut resultp1 = vec![];
    let mut resultp2 = vec![];
    let wrx1 = resultx1.clone();
    let wrx2 = resultx2.clone();
    let mut stop = tb;
    let mut sumx1 = 0.;
    let mut sumx2 = 0.;
    for i in 0..(wrx1.len()){
        sumx1 = sumx1 + wrx1[i];
        sumx2 = sumx2 + wrx2[i];
        resultp1.push(sumx1);
        resultp2.push(sumx2);
        if  t[i]>stop{
            sumx1 = 0.;
            sumx2 = 0.;
            resultp1[i] = sumx1;
            resultp2[i] = sumx2;
            stop = stop+tb;
        }
    }
    let resp1 = resultp1.clone();
    let resp2 = resultp2.clone();
    let mut resultp = vec![];
    for i in 0..resultp1.len(){
        resultp.push(resultp2[i]-resultp1[i]);
    }
    let mut wrp = resultp.clone();
    let mut resultc = vec![];
    let interval = fs*tb;
    let mut stop = interval;
    for i in 0..wrp.len(){
        if i == (stop-1.) as usize{
            if wrp[i-1] > 0.{
                for j in resultc.len()..(stop+1.) as usize{
                    resultc.push(1.);
                }
            }
            else{
                for j in resultc.len()..(stop+1.) as usize{
                    resultc.push(0.);
                }
            }
            stop = stop + interval;
        }
    }
    return (resultx1, resultx2, resp1, resp2, resultp, resultc);
}

pub fn pow2(num:i32) -> bool{
    return num & (num-1) == 0;
}

pub fn hamming_koder(bits:Vec<i32>, n:i32) -> Vec<i32>{
    let mut result = vec![0; n as usize];
    let mut idx = 0;
    for i in 0..n+1{
        if !pow2(i){
            result[(i-1) as usize] = bits[idx];
            idx = idx + 1;
        }
    }
    result[0] = result[2]^result[4]^result[6];
    result[1] = result[2]^result[5]^result[6];
    result[3] = result[4]^result[5]^result[6];
    
    return result;
}

pub fn hamming_dekoder(v1: Vec<i32>) -> (Vec<i32>, bool){
    let x1 = v1[0];
    let x2 = v1[1];
    let x4 = v1[3];
    let _x1 = v1[2]^v1[4]^v1[6];
    let _x2 = v1[2]^v1[5]^v1[6];
    let _x4 = v1[4]^v1[5]^v1[6];
    let _x1_ = x1^_x1;
    let _x2_ = x2^_x2;
    let _x4_ = x4^_x4;
    let mut detected: bool = false;
    let mut result = v1.clone();
    for _i in 0..1{
        let S = (_x1_*i32::pow(2, 0))+(_x2_*i32::pow(2, 1))+(_x4_*i32::pow(2,2));
        if S != 0{
            if v1[(S-1) as usize] == 1{
                result[(S-1) as usize] = 0;
            }
            else{
                result[(S-1) as usize] = 1;
            }
            detected = true; 
        }
        else{
            break;
        }
    }
    let mut decoded = vec![];
    for i in 0..v1.len()+1{
        if !pow2(i as i32){
            decoded.push(v1[(i-1) as usize]);
        }
    }
    return (decoded, detected);
}

pub fn draw_plot(v1: Vec<Vec<(f64, f64, f64)>>, name: &str)  -> Result<(), Box<dyn std::error::Error>> {

    let mut filename = String::from(name.clone());
    let ext = ".svg";
    filename.push_str(ext);
    let root = SVGBackend::new(&filename, (1900, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    root.titled(name, ("sans-serif", 40))?;
    
    // let (upper, lower) = root.split_vertically(450);
    

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_3d(
            (-1f64..3f64),    
            (-1f64..3f64),
            (0f64..10f64)
    
        )?;

        chart
            .configure_axes()
            .draw()?;

        chart.draw_series(
            (0..10)
                .map(|x| std::iter::repeat(x).zip(0..10))
                .flatten()
                .map(|(x, z)| {
                    Polygon::new(vec![
                        v1[x][z],
                        v1[x+1][z],
                        v1[x+1][z+1],
                        v1[x][z+1],
                    ], &BLUE.mix(0.3))
                })
            )?;
        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()?;

    Ok(())
}