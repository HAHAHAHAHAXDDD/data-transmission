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
pub use std::f64::consts::E;


pub fn multizip_dla_ubogich(v1: Vec<f64>, v2: Vec<f64>, v3:Vec<f64>) -> Vec<Vec<(f64, f64, f64)>>{
    
    let mut zipped = vec![];

    let mut indx = 0;

    for x in 0..v1.len(){
        let mut row = vec![];

        for z in 0..v3.len(){
            row.push((v1[x], v2[indx], v3[z]));       
        }

        zipped.push(row);
        indx = indx + 1;
    }

    return zipped;

}

pub fn model_systemu_transmisyjnego(bits: Vec<i32>, option: String) -> (Vec<i32>, Vec<i32>, Vec<i32>){
    let mut dec_ASK = vec![];
    let mut dec_PSK = vec![];
    let mut dec_FSK = vec![];
    if option == "1+2"{
        let names = vec!["ASK", "PSK", "FSK"];
        for name in names{
            let mut BER = vec![];
            let mut alfa_vec = vec![0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.];
            let mut beta_vec = vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
            for it in alfa_vec.clone().into_iter().zip(beta_vec.clone())
            {        
                let (alfa, beta) = it;
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
                
                //1+2
                let range = Uniform::from(-1.0..1.0);
                let rbits: Vec<f64> = rand::thread_rng().sample_iter(&range).take(210000).collect();
                let g: Vec<f64> = rbits.iter().map(|x| x*alfa).collect();
                let y_1: Vec<f64> = modulated.iter().zip(&g).map(|(x, g)| x+g).collect();
                let y: Vec<f64> = y_1.iter().zip(&t).map(|(y_1, t)| y_1*E.powf(-beta*t)).collect();
            
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
                if alfa == 0.5 && beta == 5.{
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
            }
            // let alfa_data = alfa_vec.into_iter().zip(BER.clone()).collect::<Vec<_>>();
            // let beta_data = beta_vec.into_iter().zip(BER.clone()).collect::<Vec<_>>();
            let data = multizip_dla_ubogich(alfa_vec, BER.clone(), beta_vec);
            let mut st_string = String::from("(I+II) BER dla ");
            st_string.push_str(name);
            draw_plot(data, &st_string);
        } 
    }
    else{
        let names = vec!["ASK", "PSK", "FSK"];
        for name in names{
            let mut BER = vec![];
            let mut alfa_vec = vec![0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.];
            let mut beta_vec = vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
            for it in alfa_vec.clone().into_iter().zip(beta_vec.clone())
            {        
                let (alfa, beta) = it;
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
                
                //1+2
                let range = Uniform::from(-1.0..1.0);
                let rbits: Vec<f64> = rand::thread_rng().sample_iter(&range).take(210000).collect();
                let g: Vec<f64> = rbits.iter().map(|x| x*alfa).collect();
                let y_1: Vec<f64> = modulated.iter().zip(&t).map(|(x, t)| x*E.powf(-beta*t)).collect();
                let y: Vec<f64> = y_1.iter().zip(&g).map(|(x, g)| x+g).collect();
            
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
                if alfa == 0.5 && beta == 5.{
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
            }
            // let alfa_data = alfa_vec.into_iter().zip(BER.clone()).collect::<Vec<_>>();
            // let beta_data = beta_vec.into_iter().zip(BER.clone()).collect::<Vec<_>>();
            let data = multizip_dla_ubogich(alfa_vec, BER.clone(), beta_vec);
            let mut st_string = String::from("(II+I) BER dla ");
            st_string.push_str(name);
            draw_plot(data, &st_string);
        } 
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

    //I+II
    let (dec_ASK, dec_PSK, dec_FSK) = model_systemu_transmisyjnego(bits.clone(), String::from("1+2"));
    println!("I+II");
    println!("ASK| len: {}, bity: {:?}", dec_ASK.len(), dec_ASK);
    println!("liczba bledow {}\n", bledy(bits.clone(), dec_ASK.clone()));
    println!("PSK| len: {}, bity: {:?}", dec_PSK.len(), dec_PSK);
    println!("liczba bledow {}\n", bledy(bits.clone(), dec_PSK.clone()));
    println!("FSK| len: {}, bity: {:?}", dec_FSK.len(), dec_FSK);
    println!("liczba bledow {}\n", bledy(bits.clone(), dec_FSK.clone()));

    //II+I
    let (dec_ASK, dec_PSK, dec_FSK) = model_systemu_transmisyjnego(bits.clone(), String::from("2+1"));
    println!("II+I");
    println!("ASK| len: {}, bity: {:?}", dec_ASK.len(), dec_ASK);
    println!("liczba bledow {}\n", bledy(bits.clone(), dec_ASK.clone()));
    println!("PSK| len: {}, bity: {:?}", dec_PSK.len(), dec_PSK);
    println!("liczba bledow {}\n", bledy(bits.clone(), dec_PSK.clone()));
    println!("FSK| len: {}, bity: {:?}", dec_FSK.len(), dec_FSK);
    println!("liczba bledow {}\n", bledy(bits.clone(), dec_FSK.clone()));
    
    
    // let mut data = vec![];
    
    // for x in (-2..2) {
    //     let mut row = vec![];
    //     for z in (-2..2) {
    //         row.push((x, 1, z));
    //     }
    //     data.push(row);
    // }

    // let v1 = vec![-2., -1., 0. ,1., 2.];
    // let v2 = vec![1., 1., 1., 1., 1.];
    // let v3 = vec![-2., -1., 0. ,1., 2.];

    // let test = multizip_dla_ubogich(v1, v2, v3);

    // println!("len: {}, data: {:?}", data.len(), data);
    // println!("len: {}, test {:?}", test.len(), test);

}
