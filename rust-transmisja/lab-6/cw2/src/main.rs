extern crate nalgebra as na;
pub use na::{DMatrix, DVector};
pub use rand::{distributions::Uniform, Rng};

pub fn pow2(num:i32) -> bool{
    return num & (num-1) == 0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub fn hamming_koder<const k: usize>(bits:Vec<i32>, n: i32, m:i32) -> Vec<i32>{
    let I = na::DMatrix::from_diagonal_element(k as usize, k as usize, 1);
    let b = DVector::from_vec(bits);
    let _2_0 = vec![1,0,1,0,1,0,1,0,1,0,1,0,1,0,1];
    let _2_1 = vec![0,1,1,0,0,1,1,0,0,1,1,0,0,1,1];
    let _2_2 = vec![0,0,0,1,1,1,1,0,0,0,0,1,1,1,1];
    let _2_3 = vec![0,0,0,0,0,0,0,1,1,1,1,1,1,1,1];
    let mut x1 = vec![];
    let mut x2 = vec![];
    let mut x4 = vec![];
    let mut x8 = vec![];
    for i in 1..n+1{
        if !pow2(i){
            x1.push(_2_0[(i-1) as usize]);
            x2.push(_2_1[(i-1) as usize]);
            x4.push(_2_2[(i-1) as usize]);
            x8.push(_2_3[(i-1) as usize]);
        }
    }
    let P = na::DMatrix::from_columns(   
    &[
        DVector::from_vec(x1),
        DVector::from_vec(x2),
        DVector::from_vec(x4),
        DVector::from_vec(x8),
    ]);
    let mut G = P.clone();
    G = G.insert_fixed_columns::<k>(P.ncols(), 0);
    let mut idx: usize = 0;
    for i in m..G.ncols() as i32{
        G.set_column(i as usize, &I.column(idx));
        idx = idx + 1;
    }
    let mut c = vec![];
    for j in 0..G.ncols(){
        let mut prod = 0;
        for i in 0..G.nrows(){
            if i == 0{
                prod = b[i]*G[(i,j)];
            }
            else{
                prod = prod ^ (b[i]*G[(i,j)]);
            }
        }
        c.push(prod);
    }
    return c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub fn hamming_dekoder<const k: usize>(kod: Vec<i32>, n: i32, m: i32){
    let I = na::DMatrix::from_diagonal_element(n as usize-k as usize, n as usize-k as usize, 1);
    let c = DVector::from_vec(kod);
    let _2_0 = vec![1,0,1,0,1,0,1,0,1,0,1,0,1,0,1];
    let _2_1 = vec![0,1,1,0,0,1,1,0,0,1,1,0,0,1,1];
    let _2_2 = vec![0,0,0,1,1,1,1,0,0,0,0,1,1,1,1];
    let _2_3 = vec![0,0,0,0,0,0,0,1,1,1,1,1,1,1,1];
    let mut x1 = vec![];
    let mut x2 = vec![];
    let mut x4 = vec![];
    let mut x8 = vec![];
    for i in 1..n+1{
        if !pow2(i){
            x1.push(_2_0[(i-1) as usize]);
            x2.push(_2_1[(i-1) as usize]);
            x4.push(_2_2[(i-1) as usize]);
            x8.push(_2_3[(i-1) as usize]);
        }
    }
    let mut P = na::DMatrix::from_columns(   
    &[
        DVector::from_vec(x1),
        DVector::from_vec(x2),
        DVector::from_vec(x4),
        DVector::from_vec(x8),
    ]);
    P = P.transpose();
    let mut H = I.clone();
    H = H.insert_fixed_columns::<k>(I.ncols(), 0);
    let mut idx: usize = 0;
    for i in m..H.ncols() as i32{
        H.set_column(i as usize, &P.column(idx));
        idx = idx + 1;
    }
    H = H.transpose();
    let mut s = vec![];
    for j in 0..H.ncols(){
        let mut prod = 0;
        for i in 0..H.nrows(){
            if i == 0{
                prod = c[i]*H[(i,j)];
            }
            else{
                prod = prod ^ (c[i]*H[(i,j)]);
            }
        }
        s.push(prod);
    }

    let mut pos: i32 = 0;
    let mut detected: bool = false;
    let mut cor = c.clone();
    for _i in 0..1{
        let S = (s[0]*i32::pow(2, 0))+(s[1]*i32::pow(2, 1))+(s[2]*i32::pow(2,2))+(s[3]*i32::pow(2,3));
        if S != 0{
            pos = S;
            if cor[(S-1) as usize] == 1{
                cor[(S-1) as usize] = 0;
            }
            else{
                cor[(S-1) as usize] = 1;
            }
            detected = true; 
        }
        else{
            break;
        }
    }
    if detected == true{
        println!("Wiadomosc otrzymana: {} \nZnaleziono blad: {} (pozycja: {})\nWiadomosc poprawiona: {}", c.transpose(), detected, pos, cor.transpose());
    }
    else{
        println!("Wiadomosc otrzymana: {} \nZnaleziono blad: {}", c.transpose(), detected);
    }
}

#[allow(non_upper_case_globals)]
fn main() {
    // let range = Uniform::from(0..2);
    // let bits: Vec<i32> = rand::thread_rng().sample_iter(&range).take(11).collect();
    
    let n: i32 = 15;
    const k: usize = 11;
    let m: i32 = 4;

    //Wiadomosc poprawna
    let bits = vec![1,1,0,0,1,0,0,1,0,1,0];
    let koder = hamming_koder::<k>(bits, n, m);
    println!("Wiadomosc zakodowana: {:?}", koder);
    hamming_dekoder::<k>(koder, n ,m);
    
    println!("\n");

    //Wiadomosc bledna
    let bits = vec![1,1,0,0,1,0,0,1,0,1,0];
    let mut koder = hamming_koder::<k>(bits, n, m);
    println!("Wiadomosc zakodowana: {:?}", koder);
    koder[12] = 1;
    hamming_dekoder::<k>(koder, n ,m);
}
