pub fn lebih_besar_mana(a: i32, b: i32) -> i32{
    if a > b {return a}
    b
}

pub fn apakah_ganjil(a: i32) -> bool{
    if a % 2 == 0 {return false;}
    true
}

pub fn berapa_jumlahnya(a: u32) -> u32{
    let mut hasil = 0;
    for val in 0..=a {
        hasil += val;
    }
    hasil
}

pub fn ganjil_genap_dua(len: i32, val: Vec<i32>) -> Vec<String>{
    if len != val.len() as i32{panic!("lenght doesn't match length argument");}
    let mut result: Vec<String> = vec![];
    let mut ganjilapagenap: String;
    for v in val {
        ganjilapagenap = match (v % 2) == 0 {
            true => "Genap".to_string(),
            _ => "Ganjil".to_string()
        };
        result.push(ganjilapagenap)
    }
    result
}

pub fn lebih_besar_mana_dua(len: i32, val: Vec<i32>) -> i32 {
    if len != val.len() as i32 {panic!("length value doesn't match length argument")}

    let mut result: i32 = *val.get(0).unwrap();
    for v in val {
        if v > result{
            result = v;
        }
    }
    result
}

pub fn mungkinkah(len: i32, k: i32, val: Vec<i32>) -> String{
    if len != val.len() as i32 {panic!("length value doesn't match length argument")}

    for (i, v) in val.clone().iter().enumerate() {
        for (j, w) in val.clone().iter().enumerate() {
            if (v + w == k) && i != j{
                return "Ada".to_string();
            }
        }
    }
    "Tidak Ada".to_string()
}

pub fn apa_semua_beda(len: i32, val: Vec<i32>) -> String {
    if len != val.len() as i32 {panic!("length value doesn't match length argument")}

    for (i, v) in val.clone().iter().enumerate() {
        for (j, w) in val.clone().iter().enumerate() {
            if (v == w) && i != j{
                return "Ada yang sama".to_string();
            }
        }
    }
    "Semua berbeda".to_string()
}

pub fn penjualan_buah(val: Vec<(String, i32)>) -> String {
    let mut result: String = String::new();
    let mut max_val: i32 = 0;

    for v in val.clone(){
        if v.1 > max_val{
            max_val = v.1;
        }
    }

    for v in val.clone(){
        if v.1 == max_val{
            result.push_str(&v.0);
            result.push_str(" ");
        }
    }
    result
}
