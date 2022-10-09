use std::collections::HashMap;
use std::fmt;
use std::fs;
use bitvec::prelude::*;


fn main() {
    //TODO read from cli
    let name = "Николаев Г.В.".to_owned();

    //encodes name with given table
    let code = encode(&name);

    //prints info
    println!("Исходное сообщение: \"{}\"",&name);
    //V is a wierd workaround to print in hex and binary found on stackoverflow
    //https://stackoverflow.com/questions/54042984/can-i-format-debug-output-as-binary-when-the-values-are-in-a-vector
    println!("в шестнадцатеричном коде: {:X}",V(&code));
    println!("в двоичном коде: {:b}",V(&code));
    println!("длина сообщения: {} байт ({} бит)",code.len(),code.len()*8);

    //sending first for byte to manchester encoding and geting a csv string
    let manchester = manchester_enc(&code[0..4]);
    //writing csv string to file lines 28..40 are analogues 
    fs::write("./manchester.csv", manchester).expect("Can't write to manchester.csv");
    println!("Written to manchester.csv ");

    let nrz = nrz_enc(&code[0..4]);
    fs::write("./nrz.csv", nrz).expect("Can't write to nrz.csv");
    println!("Written to nrz.csv ");

    let rz = rz_enc(&code[0..4]);
    fs::write("./rz.csv", rz).expect("Can't write to rz.csv");
    println!("Written to rz.csv ");

    let ami = ami_enc(&code[0..4]);
    fs::write("./ami.csv", ami).expect("Can't write to ami.csv");
    println!("Written to ami.csv ");

    let mlt = mlt_enc(&code[0..4]);
    fs::write("./mlt.csv", mlt).expect("Can't write to mlt.csv");
    println!("Written to mlt.csv ");

    /* Doesn't work properly
    let sequence = format!("{:b}",V(&code));
    let sequence: String = sequence.split_whitespace().collect();
    let sequence: Vec<char> = sequence.chars().collect();
    let mut count_vec = vec![0;sequence.len()];
    let mut counter = 0;
    let mut max = 1;
    for i in 1..(sequence.len()) {
        if sequence[i-1] == sequence[i]{
            max += 1;
        } else {
            count_vec[max-1] += 1;
            if max > counter {
                counter = max;
            }
            max = 1;
        }
    }
    println!("longest sequence: {}",counter);


    let c = 1_000_000;
    let nrz_f_v = c/2; 
    println!("nrz_f_v {nrz_f_v}");
    let nrz_f_n = c/(counter*2);
    println!("nrz_f_n {nrz_f_n}");
    let nrz_s = nrz_f_v - nrz_f_n;
    println!("nrz_s {nrz_s}");
    let mut nrz_f_av: usize = 0;
    let nrz_f0 = c/2;
    println!("{:?}",count_vec);
    for i in 0..counter {
        nrz_f_av = count_vec[i]/(i+1);
    }
    let nrz_f_av = ((nrz_f_av*nrz_f0) as f64) / (sequence.len() as f64);
    println!("nrz_f_av {nrz_f_av}");
    println!("nrz_s {nrz_s}");




    let rz_f_v = c;
    let rz_f_n = c/4;
    let rz_s = rz_f_v - rz_f_n;

    let manchester_f_v = c;
    let manchester_f_n = c/2;
    let manchester_s = manchester_f_v - manchester_f_n;
    */
    




    //encoding the code logically
    let log_code = log_encode(&code);
    //calculating lenght
    let log_len = code.len() as f32*1.25;

    println!("в шестнадцатеричном коде: {:X}",V(&log_code));
    println!("в двоичном коде: {:b}",V(&log_code));
    println!("длина сообщения: {} байт ({} бит)",log_len,log_len*8.);

    //writing csvs
    let log_manchester = manchester_enc(&log_code[0..4]);
    fs::write("./log_manchester.csv", log_manchester).expect("Can't write to log_manchester.csv");

    let log_ami = ami_enc(&log_code[0..4]);
    fs::write("./log_ami.csv", log_ami).expect("Can't write to log_ami.csv");

    //scrambling code
    let mut scam5_code = code.clone();
    scramble_5(&mut scam5_code);

    println!("в шестнадцатеричном коде: {:X}",V(&scam5_code));
    println!("в двоичном коде: {:b}",V(&scam5_code));
    println!("длина сообщения: {} байт ({} бит)",scam5_code.len(),scam5_code.len()*8);

    //scrambling with another polynome
    let mut scam7_code = code.clone();
    scramble_7(&mut scam7_code);

    println!("в шестнадцатеричном коде: {:X}",V(&scam7_code));
    println!("в двоичном коде: {:b}",V(&scam7_code));
    println!("длина сообщения: {} байт ({} бит)",scam7_code.len(),scam7_code.len()*8);

    //writing csvs
    let scam_manchester = manchester_enc(&scam7_code[0..4]);
    fs::write("./scam_manchester.csv", scam_manchester).expect("Can't write to scam_manchester.csv");

    let scam_ami = ami_enc(&scam7_code[0..4]);
    fs::write("./scam_ami.csv", scam_ami).expect("Can't write to scam_ami.csv");
}

//scrambling function with polynome B_i = A_i XOR B_i-3 XOR B_i-5
fn scramble_5(code: &mut [u8]) {
    let bits = code.view_bits_mut::<Msb0>();

    bits.set(3,bits[3]^bits[0]);
    bits.set(4,bits[4]^bits[1]);

    for i in 5..bits.len() {
        bits.set(i,bits[i]^bits[i-3]^bits[i-5])
    }
}

//scrambling function with polynome B_i = A_i XOR B_i-5 XOR B_i-7
fn scramble_7(code: &mut [u8]) {
    let bits = code.view_bits_mut::<Msb0>();

    bits.set(5,bits[3]^bits[0]);
    bits.set(6,bits[4]^bits[1]);

    for i in 7..bits.len() {
        bits.set(i,bits[i]^bits[i-5]^bits[i-7])
    }
}


//function that writes csv for mlt-3 graph
fn mlt_enc(code: &[u8]) -> String {
    let mut time = 0;
    let mut counter = 0;
    let mut string = String::from("MLT-3;\n");
    for byte in code.into_iter() {
        for j in (0..8).rev() {
            let s;
            if counter%4 == 0 {
                s = -1;
            } else if counter%2 == 0 {
                s = 1;
            } else {
                s = 0;
            }
            if 1 & byte>>j == 1 {
                string = format!("{2}{0};{3}\n{1};{3}\n",time,(time+1),string,s);
                counter += 1;
            } else {
                string = format!("{2}{0};{3}\n{1};{3}\n",time,(time+1),string,s);
            }
            time += 1;
        }
    }
    string
}

//function that writes csv for ami graph
fn ami_enc(code: &[u8]) -> String {
    let mut time = 0;
    let mut sign = true;
    let mut string = String::from("AMI;\n");
    for byte in code.into_iter() {
        for j in (0..8).rev() {
            if 1 & byte>>j == 1 {
                let s;
                if sign {
                    s = '-';
                } else {
                    s = ' ';
                } string = format!("{2}{0};{3}1\n{1};{3}1\n",time,(time+1),string,s);
                sign = !sign;
            } else {
                string = format!("{2}{0};0\n{1};0\n",time,(time+1),string);
            }
            time += 1;
        }
    }
    string
}

//function that writes csv for rz graph
fn rz_enc(code: &[u8]) -> String {
    let mut time = 0;
    let mut string = String::from("RZ;\n");
    for byte in code.into_iter() {
        for j in (0..8).rev() {
            if 1 & byte>>j == 1 {
                string = format!("{2}{0};1\n{0}.5;1\n{0}.5;0\n{1};0\n",time,(time+1),string);
            } else {
                string = format!("{2}{0};-1\n{0}.5;-1\n{0}.5;0\n{1};0\n",time,(time+1),string);
            }
            time += 1;
        }
    }
    string
}

//function that writes csv for manchester graph
fn manchester_enc(code: &[u8]) -> String {
    let mut time = 0;
    let mut string = String::from("Manchester code;\n");
    for byte in code.into_iter() {
        for j in (0..8).rev() {
            if 1 & byte>>j == 1 {
                string = format!("{2}{0};1\n{0}.5;1\n{0}.5;0\n{1};0\n",time,(time+1),string);
            } else {
                string = format!("{2}{0};0\n{0}.5;0\n{0}.5;1\n{1};1\n",time,(time+1),string);
            }
            time += 1;
        }
    }
    string
}

//function that writes csv for nrz graph
fn nrz_enc(code: &[u8]) -> String {
    let mut time = 0;
    let mut string = String::from("NRZ;\n");
    for byte in code.into_iter() {
        for j in (0..8).rev() {
            if 1 & byte>>j == 1 {
                string = format!("{2}{0};1\n{1};1\n",time,(time+1),string);
            } else {
                string = format!("{2}{0};0\n{1};0\n",time,(time+1),string);
            }
            time += 1;
        }
    }
    string
}

//function that logically encodes code with given table
fn log_encode(code: &[u8]) -> Vec<u8> {
    //dictionary for converting 
    let dict: HashMap<u8, u8> = HashMap::from([
        (0b0000, 0b11110),
        (0b0001, 0b01001),
        (0b0010, 0b10100),
        (0b0011, 0b10101),
        (0b0100, 0b01010),
        (0b0101, 0b01011),
        (0b0110, 0b01110),
        (0b0111, 0b01111),
        (0b1000, 0b10010),
        (0b1001, 0b10011),
        (0b1010, 0b10110),
        (0b1011, 0b10111),
        (0b1100, 0b11010),
        (0b1101, 0b11011),
        (0b1110, 0b11100),
        (0b1111, 0b11101),
    ]);
    let len = code.len();
    let mut code = code.into_iter();
    let mut encoded: Vec<u8> = Vec::with_capacity(len);
    for _ in 0..(len/4) {
        let mut a: u64 = 0;
        for _ in (0..4).rev() {
            let byte = code.next().unwrap();
            let shalf = byte & 0xF;
            let fhalf = (byte & 0xF0)>>4;
            let tenbit =  ((dict[&fhalf] as u64)<<5) | (dict[&shalf] as u64);
            a = a<<10 | tenbit;
        }
        a = a << 24;
        let a = a.to_be_bytes();
        for j in 0..5 {
            encoded.push(a[j]);
        }
    }
    let reminder = len%4;
    let mut a: u64 = 0;
    for i in 0..reminder {
        let byte = code.next().unwrap();
        let shalf = byte & 0xF;
        let fhalf = (byte & 0xF0)>>4;
        let tenbit =  ((dict[&fhalf] as u64)<<5) | (dict[&shalf] as u64);
        a = a | ((tenbit) << (64-(10*(i+1))));
    }
    let a = a.to_be_bytes();
    let size = reminder as f64 * 1.25;
    for i in 0..(size.ceil() as usize) {
        encoded.push(a[i]);
    }
    encoded
}

//function that encodes string with given table
fn encode(string: &String) -> Vec<u8> {
    let dict: HashMap<char, u8> = HashMap::from([
        ('А', 0xC0),
        ('Б', 0xC1),
        ('В', 0xC2),
        ('Г', 0xC3),
        ('Д', 0xC4),
        ('Е', 0xC5),
        ('Ж', 0xC6),
        ('З', 0xC7),
        ('И', 0xC8),
        ('Й', 0xC9),
        ('К', 0xCA),
        ('Л', 0xCB),
        ('М', 0xCC),
        ('Н', 0xCD),
        ('О', 0xCE),
        ('П', 0xCF),
        ('Р', 0xD0),
        ('С', 0xD1),
        ('Т', 0xD2),
        ('У', 0xD3),
        ('Ф', 0xD4),
        ('Х', 0xD5),
        ('Ц', 0xD6),
        ('Ч', 0xD7),
        ('Ш', 0xD8),
        ('Щ', 0xD9),
        ('Ъ', 0xDA),
        ('Ы', 0xDB),
        ('Ь', 0xDC),
        ('Э', 0xDD),
        ('Ю', 0xDE),
        ('Я', 0xDF),
        ('а', 0xE0),
        ('б', 0xE1),
        ('в', 0xE2),
        ('г', 0xE3),
        ('д', 0xE4),
        ('е', 0xE5),
        ('ж', 0xE6),
        ('з', 0xE7),
        ('и', 0xE8),
        ('й', 0xE9),
        ('к', 0xEA),
        ('л', 0xEB),
        ('м', 0xEC),
        ('н', 0xED),
        ('о', 0xEE),
        ('п', 0xEF),
        ('р', 0xF0),
        ('с', 0xF1),
        ('т', 0xF2),
        ('у', 0xF3),
        ('ф', 0xF4),
        ('х', 0xF5),
        ('ц', 0xF6),
        ('ч', 0xF7),
        ('ш', 0xF8),
        ('щ', 0xF9),
        ('ъ', 0xFA),
        ('ы', 0xFB),
        ('ь', 0xFC),
        ('э', 0xFD),
        ('ю', 0xFE),
        ('я', 0xFF),
        (' ', 0x20),
        (',', 0x2C),
        ('.', 0x2E),
        ('0', 0x30),
        ('1', 0x31),
        ('2', 0x32),
        ('3', 0x33),
        ('4', 0x34),
        ('5', 0x35),
        ('6', 0x36),
        ('7', 0x37),
        ('8', 0x38),
        ('9', 0x39),
    ]);
    let chars = string.clone();
    let mut encoded: Vec<u8> = Vec::with_capacity(string.chars().count());
    for i in chars.chars() {
        encoded.push(dict[&i]);
    }
    encoded
}

//copied from github to print arrays in binary
struct V<'a>(&'a Vec<u8>);

// custom output
impl fmt::Binary for V<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // extract the value using tuple idexing
        // and create reference to 'vec'
        let vec = &self.0;

        // @count -> the index of the value,
        // @n     -> the value
        for (count, n) in vec.iter().enumerate() { 
            if count != 0 { write!(f, " ")?; }

            write!(f, "{:08b}", n)?;
        }

        Ok(())
    }
}

//changed binary trait to print UpperHex
impl fmt::UpperHex for V<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // extract the value using tuple idexing
        // and create reference to 'vec'
        let vec = &self.0;

        // @count -> the index of the value,
        // @n     -> the value
        for (count, n) in vec.iter().enumerate() { 
            if count != 0 { write!(f, " ")?; }

            write!(f, "{:X}", n)?;
        }

        Ok(())
    }
}
