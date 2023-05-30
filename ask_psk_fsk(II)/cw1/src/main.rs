pub use iter_num_tools::lin_space;
extern crate iter_num_tools;
pub use euclid::*;
pub use std::f64::consts::PI;

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
    let h = 510.;
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


fn main() {
    let mut bin = ascii_to_bin("Elo");
    let mut slice = &bin[0..10];
    let mut b = vec![];
    for i in 0..slice.len(){
        b.push(slice[i]);
    }
    let n: f64 = b.len() as f64 - 1.;
    let bclone = b.clone();
    let B: f64 = bclone.len() as f64;

    let W: f64 = 2.;
    let fs: f64 = b.len() as f64 * 1000.; //czestotliwosc
    let tc: f64 = 1.; //czas symulacji
    let N: f64 = tc*fs;
    let tb: f64 = tc/B;
    let _fn: f64 = W*(1./tb);
    let a1: f64 = 1.;
    let a2: f64 = 2.;
    let i = lin_space(0.0..=N, N as usize);
    let mut t = vec![];
    for j in i{
        t.push(j/fs);
    }
    //ASK
    let task = t.clone();
    let bclone = b.clone();
    let ask = ASK(task, bclone, _fn, fs, a1, a2); 
    let tclone = t.clone();
    let (resultx, resultp, resultc) = dASK(ask, tclone, _fn, a1, fs, tb);

    //PSK
    let tpsk = t.clone();
    let bclone = b.clone();
    let res = PSK(tpsk, bclone, _fn, fs);
    let tclone = t.clone();
    let (x,y,z) = dPSK(res, tclone, _fn, a1, fs, tb);
    
    //FSK
    let _fn1 = (n+1.)/tb;
    let _fn2 = (n+2.)/tb;
    let tclone = t.clone();
    let bclone = b.clone();
    let fsk = FSK(tclone, bclone, _fn1, _fn2, fs);
    let tclone = t.clone();
    let (x1, x2, p1, p2, p, c) = dFSK(fsk, tclone, _fn1, _fn2, a1, fs, tb); 
}
