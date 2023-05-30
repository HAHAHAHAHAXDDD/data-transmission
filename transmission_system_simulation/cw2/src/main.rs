#![allow(non_snake_case, 
    unused_variables, 
    unused_mut, 
    non_upper_case_globals,
    unused_must_use,
    unused_assignments
)]
#[path="../funkcje.rs"]
mod funkcje;
pub use funkcje::{
    ascii_to_bin, ASK, PSK, 
    FSK, dASK, dPSK, 
    dFSK, pow2, hamming_koder,
    hamming_dekoder, draw_plot
};
pub use rand::{distributions::Uniform, Rng};
pub use iter_num_tools::lin_space;

pub fn model_systemu_transmisyjnego(bits: Vec<i32>) -> (Vec<i32>, Vec<i32>, Vec<i32>){
    let mut dec_ASK = vec![];
    let mut dec_PSK = vec![];
    let mut dec_FSK = vec![];
    let names = vec!["ASK", "PSK", "FSK"];
    for name in names{
        let mut BER = vec![];
        let mut alfa_vec = vec![0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.];
        for alfa in alfa_vec.clone()
        {        
            let mut koder_nadmiarowy = vec![];
            
            // Hamming(7, 4)
            let mut start = 0;
            let mut stop = 4;
            for i in 0..(bits.len()/4){
                koder_nadmiarowy.push(hamming_koder((&bits[start..stop]).to_vec(), 7));
                    start = stop;
                stop += 4;
            }
        
            // Zmiana paczek na jeden wektor
            let mut koder = koder_nadmiarowy.into_iter().flatten().collect::<Vec<i32>>();
            // println!("len: {}, koder: {:?}", koder.len(), koder);
            
            // Dane do modulacji
            let B: f64 = koder.len() as f64;
            let fs: f64 = B * 1000.;
            let tc: f64 = 1.;
            let tb: f64 = tc/B;
            let _fn: f64 = 2.*(1./tb);
            let N: f64 = tc*fs;
            let i = lin_space(0.0..=N, N as usize);
            let a: f64 = 1.;
            let mut t = vec![];
            for j in i{
                t.push(j/fs);
            }
            let mut modulated = vec![];
            // Modulacja
            if name == "ASK"{
                let a1 = 1.;
                let a2 = 0.5;
                modulated = ASK(t.clone(), koder, _fn, fs, a1, a2);
            }
            else if name == "PSK" {
                modulated = PSK(t.clone(), koder, _fn, fs);
            }
            else{
                let _fn1 = (bits.clone().len() as f64+1.)/tb;
                let _fn2 = (bits.clone().len() as f64+2.)/tb;
                modulated = FSK(t.clone(), koder, _fn1, _fn2, fs);
            }
            // println!("zmodulowane bity: {:?}", modulated);
            
            let range = Uniform::from(-1.0..1.0);
            let rbits: Vec<f64> = rand::thread_rng().sample_iter(&range).take(210000).collect();
            let g: Vec<f64> = rbits.iter().map(|x| x*0.4).collect();
            let y: Vec<f64> = modulated.iter().zip(&g).map(|(modulated, g)| modulated+g).collect();
        
            // Demodulacja zwrócenie x, p, c (przy czym tak wlasciwie wykorzystane tylko c)
            let mut x = vec![];
            let mut p = vec![];
            let mut c = vec![];
            
            if name == "ASK"{
                (x, p, c) = dASK(y.clone(), t.clone(), _fn, a, fs, tb);
            }
            else if name == "PSK" {
                (x, p, c) = dPSK(y.clone(), t.clone(), _fn, a, fs, tb);
            }
            else{
                let _fn1 = (bits.clone().len() as f64+1.)/tb;
                let _fn2 = (bits.clone().len() as f64+2.)/tb;
                let mut x1 = vec![];
                let mut x2 = vec![];
                let mut p1 = vec![];
                let mut p2 = vec![];
                let mut p = vec![];
                (x1, x2, p1, p2, p, c) = dFSK(y.clone(), t.clone(), _fn1, _fn2, a, fs, tb);
            }
            let mut demodulated = vec![];
            for i in (1..c.len()).step_by(1000){
                demodulated.push(c[i] as i32);
            }
            //Hamming dekoder
            let mut decoded_p = vec![];
            let mut errors = vec![];
            let mut start = 0;
            let mut stop = 7;
            for i in 0..(demodulated.len()/7){
                let (dec, det) = hamming_dekoder((&demodulated[start..stop]).to_vec());
                decoded_p.push(dec);
                errors.push(det);
                start = stop;
                stop += 7;
            }
            // zlaczenie paczek w jeden wynikowy wektor
            let mut decoded = decoded_p.into_iter().flatten().collect::<Vec<i32>>();
        
            // bledy dla danych paczek bitow   
            let bledy = bits.clone().iter().zip(&decoded.clone())
            .filter(|&(a, b)| a != b).count();
            BER.push(bledy as f64/bits.len() as f64);       
            if name == "ASK"{            
                dec_ASK = decoded.clone();
            }    
            else if name == "PSK"{
                dec_PSK = decoded.clone();
            }
            else{
                dec_FSK = decoded.clone();
            }
        }
        let data = alfa_vec.into_iter().zip(BER.clone()).collect::<Vec<_>>();
        let mut st_string = String::from("BER dla ");
        st_string.push_str(name);
        draw_plot(data, 0., 1., 0.3, 0.6, st_string);
    }
    return(dec_ASK, dec_PSK, dec_FSK);
}

fn main() {
    // liczenie bledow
    let bledy = |bits: Vec<i32>, decoded: Vec<i32>| -> usize{
        bits.iter().zip(&decoded)
        .filter(|&(a, b)| a != b).count()
    };


    let mut bits = ascii_to_bin("test jest robiony");
    bits.push(1);
    println!("len: {}, otrzymane bity: {:?}\n", bits.len(), bits);

    let (dec_ASK, dec_PSK, dec_FSK) = model_systemu_transmisyjnego(bits.clone());
    println!("ASK| len: {}, bity: {:?}", dec_ASK.len(), dec_ASK);
    println!("liczba bledow {}\n", bledy(bits.clone(), dec_ASK.clone()));
    println!("PSK| len: {}, bity: {:?}", dec_PSK.len(), dec_PSK);
    println!("liczba bledow {}\n", bledy(bits.clone(), dec_PSK.clone()));
    println!("FSK| len: {}, bity: {:?}", dec_FSK.len(), dec_FSK);
    println!("liczba bledow {}\n", bledy(bits.clone(), dec_FSK.clone()));
}
