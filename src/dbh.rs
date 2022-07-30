use num::pow;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != *&3 as usize {
        println!("incorrect args");
    } else {
        if args[1] == String::from("hb") {      
            println!("{}", hex2bin(String::from(&args[2])));
        }   
        else if args [1] == String::from("hd") {
            println!("{}", hex2dec(String::from(&args[2])));
        }   
        else if args [1] == String::from("bd") {
            println!("{}", bin2dec(String::from(&args[2])));
        }   
        else if args [1] == String::from("bh") {
            println!("{}", bin2hex(String::from(&args[2])));
        }   
        else if args [1] == String::from("db") {
            println!("{}", dec2bin(String::from(&args[2])));
        }   
        else if args [1] == String::from("dh") {
            println!("{}", dec2hex(String::from(&args[2])));
        } else {
            println!("{}", "wrong".to_string());
        }
    }
}

#[allow(non_snake_case)]
fn hexA() -> [&'static str; 16] {
    return ["0","1","2","3","4","5","6","7","8","9","A","B","C","D","E","F"];
}

#[allow(non_snake_case)]
fn binA() -> [&'static str; 16] {
    return ["0000","0001","0010","0011","0100","0101","0110","0111","1000","1001","1010","1011","1100","1101","1110","1111"];
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn decA() -> [&'static str; 16] {
    return ["0","1","2","3","4","5","6","7","8","9","10","11","12","13","14","15"];
}

// Hexadecimal to binary
fn hex2bin (hex: String) -> String {
    let chars: Vec<char> = hex.to_uppercase().chars().collect();
    let mut bin: String = "".to_string();
    for i in chars {
        bin.push_str(binA()[hexA().iter().position(|&n| n == &i.to_string()).unwrap()]);
    }
    return bin;
}

// Hexadecimal to decimal
fn hex2dec (hex: String) -> String {
    return bin2dec(hex2bin(hex));
}


// Binary to decimal
fn bin2dec (bin: String) -> String {
    let chars: Vec<char> = bin.chars().collect();
    let mut cur =  pow(2u16, chars.len() - 1);
    let mut total: u16 = 0;
    for i in chars {
        match i {
            '0' => total += 0,
            '1' => total += cur,
            _ => println!("{}", String::from("no")),
        }
        cur /= 2;
    }
    return total.to_string();
}

// Binary to hexadecimal
fn bin2hex(bin: String) -> String {
    let chars: Vec<char> = bin.chars().collect();
    let mut placed: u8 = 0;
    let mut cur: String = "".to_string();
    let mut fin: String = "".to_string();
    for i in chars {
        cur.push_str(&i.to_string());
        placed += 1;
            if placed >= 4 {
                cur = hexA()[binA().iter().position(|&n| n == &cur).unwrap()].to_string();
                fin.push_str(&cur);
                cur = "".to_string();
                placed = 0;
        }
    }
    return fin;
}

// Decimal to binary
fn dec2bin(dec: String) -> String {
    let num: u32 = dec.parse().unwrap();
    let mut max: u32 = 0;
    let mut cur: u32 = 0;
    let mut fin: String = "".to_string();
    
    if &num <= &255 {
        max = 128;
    } else if &num <= &65535 {
        max = 32768;
    } else if &num <= &4294967295 {
        max = 2147483648;
    }

    while &mut max >= &mut 1 {
        if &(&cur + &max) <= &num {
            fin.push_str("1");
            cur += max;
        } else {
            fin.push_str("0");
        }
        max /= 2;
    }
    return fin;
}

// Decimal to hexadecimal
fn dec2hex(dec: String) -> String {
    return bin2hex(dec2bin((&dec).to_string()));
}
