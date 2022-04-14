use std::{ process, io };

mod soal;

pub fn run(config: Config){
    match config.no {
        1 => {
            if config.input.len() > 2 {panic!("too many arguments");}
            let a: i32 = config.input.get(0).expect("invalid argument").parse().expect("argument must be an integer");
            let b: i32 = config.input.get(1).expect("invalid argument").parse().expect("argument must be an integer");
            println!("lebih besar {}", soal::lebih_besar_mana(a, b));
        }

        2 => {
            if config.input.len() > 1 {panic!("too many arguments");}
            let a: i32 = config.input.get(0).expect("invalid argument").parse().expect("argument must be an integer");
            if soal::apakah_ganjil(a){
                println!("{} adalah bilangan ganjil", a);
            }else{
                println!("{} adalah bilangan genap", a);
            }
        }

        3 => {
            if config.input.len() > 1 {panic!("too many arguments");}
            let a: i32 = config.input.get(0).expect("invalid argument").parse().expect("argument must be an integer");
            println!("hasil penjumlahan dari {} adalah {}",a , soal::berapa_jumlahnya(a as u32));

        }

        4 => {
            let mut a: Vec<i32> = vec![];
            for val in config.input {
                a.push(val.parse().expect("invalid argument(s)"));
            }
            println!("{:?}", soal::ganjil_genap_dua(a.len() as i32, a));
        }

        5 => {
            let mut a: Vec<i32> = vec![];
            for val in config.input {
                a.push(val.parse().expect("invalid argument(s)"));
            }
            println!("Bilangan yang paling besar adalah {}", soal::lebih_besar_mana_dua(a.len() as i32, a));
        }

        6 => {
            let mut a = config.input.iter();
            let mut args: Vec<i32> = vec![];
            let k: i32= a.next().expect("not enough argument").parse().expect("invalid argument(s)");
            for val in a {
                args.push(val.parse().expect("invalid argument(s)"));
            }
            println!("{}", soal::mungkinkah(args.len() as i32, k, args));
        }

        7 => {
            let mut a: Vec<i32> = vec![];
            for val in config.input {
                a.push(val.parse().expect("invalid argument(s)"));
            }
            println!("{}", soal::apa_semua_beda(a.len() as i32, a));
        }

        8 => {
            let mut input: Vec<(String, i32)> = vec![];

            let mut a = config.input.iter();
            let mut string_buffer = String::new();
            let mut int_buffer: i32 = 0;
            'a: loop {
                match a.next() {
                    Some(x) => {
                        string_buffer = x.to_string();
                    }
                    None => break 'a
                }
                match a.next() {
                    Some(x) => {
                        int_buffer = x.parse().expect("invalid input");
                    }
                    None => break 'a
                }
                input.push((string_buffer, int_buffer));
            }

            println!("buah yang paling banyak adalah {}", soal::penjualan_buah(input));
        }

        _ => {
            eprintln!("invalid argument(s)");
            process::exit(1);
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub no: u32,
    pub input: Vec<String>
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let no: u32 = args[1].clone().parse().expect("invalid input");
        let mut input: Vec<String> = vec![];
        for (i, el) in args.iter().enumerate(){
            if i < 2 {continue;}
            input.push(el.clone());
        }
        Ok(Config {
            no,
            input
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::soal;

    #[test]
    fn lima(){
        assert_eq!(soal::lebih_besar_mana_dua(3, vec![3, 4, 5]), 5);
    }

    #[test]
    fn delapan(){
        assert_eq!("jeruk ".to_string(), soal::penjualan_buah(vec![("jeruk".to_string(), 90), ("pepaya".to_string(), 29)]));
    }
}