use std::io::{stdin, Read};

fn main(){
    let mut c: char;
    //BufReader()
    //let mut stdin: io::stdin();
    let mut buffor: [u8; 1] = [0];
    loop {
        match stdin().read_exact(&mut buffor) {
            Ok(_) => {c = buffor[0] as char},
            Err(_) => {break}
        }
        print!("{}", c);
    }
}