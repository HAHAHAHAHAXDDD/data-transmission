#![allow(non_snake_case, 
    unused_variables, 
    unused_mut, 
    non_upper_case_globals,
    unused_must_use
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

pub fn model_systemu_transmisyjnego(bits: Vec<i32>) -> (Vec<i32>, Vec<bool>){
    // println!("len: {}, otrzymane bity: {:?}", bits.len(), bits);
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

    // Modulacja
    let mut modulated = PSK(t.clone(), koder, _fn, fs);
    
    // Demodulacja zwrócenie x, p, c (przy czym tak wlasciwie wykorzystane tylko c)
    let (x, p, c) = dPSK(modulated.clone(), t.clone(), _fn, a, fs, tb);
    let mut demodulated = vec![];
    for i in (1..c.len()).step_by(1000){
        demodulated.push(c[i] as i32);
    }
    // println!("len: {}, demodulated: {:?}", demodulated.len(), demodulated);

    //Hammin dekoder
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
    // println!("len: {},  decoded: {:?}", decoded.len(), decoded);

    // bledy dla danych paczek bitow
    // println!("tablica bledow: {:?}", errors);
    return(decoded, errors)
}

fn main() {
    // let range = Uniform::from(0..2);
    // let bits: Vec<i32> = rand::thread_rng().sample_iter(&range).take(120).collect();
    
    let mut bits = ascii_to_bin("test jest robiony");
    bits.push(1);
    println!("len: {}, otrzymane bity: {:?}", bits.len(), bits);

    let(result, errors) = model_systemu_transmisyjnego(bits.clone());
    println!("len: {},  wynik: {:?}", result.len(), result.clone());
    
    // hamming distance
    let bledy = bits.clone().iter().zip(&result.clone())
    .filter(|&(a, b)| a != b).count();
    println!("liczba bledow: {}", bledy);
}
