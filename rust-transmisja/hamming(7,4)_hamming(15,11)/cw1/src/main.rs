pub fn pow2(num:i32) -> bool{
    return num & (num-1) == 0;
}

pub fn hamming_koder(bits:Vec<i32>, n:i32) -> Vec<i32>{
    let mut result = vec![0; n as usize];
    let mut idx = 0;
    for i in 0..n{
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

#[allow(non_snake_case, non_upper_case_globals)]
pub fn hamming_dekoder(v1: Vec<i32>){
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
    let mut pos = 0;
    for _i in 0..1{
        let S = (_x1_*i32::pow(2, 0))+(_x2_*i32::pow(2, 1))+(_x4_*i32::pow(2,2));
        if S != 0{
            pos = S;
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
    if detected == true{
        println!("Wiadomosc otrzymana: {:?} \nZnaleziono blad: {} (pozycja: {})\nWiadomosc poprawiona: {:?}\nWiadomosc zdekodowana: {:?}", v1, detected, pos, result, decoded);
    }
    else{
        println!("Wiadomosc otrzymana: {:?} \nZnaleziono blad: {}\nWiadomosc zdekodowana: {:?}", v1, detected, decoded);
    }
}

fn main() {

    //Wiadomosc poprawna
    let bits = vec![1,0,1,0];
    let koder = hamming_koder(bits, 7);
    println!("Wiadomosc zakodowana: {:?}", koder);
    hamming_dekoder(koder); 
    
    println!("\n");

    //Wiadomosc bledna
    let bits = vec![1,0,1,0];
    let mut koder = hamming_koder(bits, 7);
    println!("Wiadomosc zakodowana: {:?}", koder);
    koder[3] = 0;
    hamming_dekoder(koder);
}
