use std::{io::{stdin, Read}, os::fd};
use termios::*;

fn change_flags(){
    let fd = 0;
    let mut termios = Termios::from_fd(fd).unwrap();
    match tcgetattr(fd, &mut termios){
        Err(why) => panic!("Error while putting terminal in raw mode: {}", why),
        Ok(_) => {}
    }

    termios.c_lflag &= !(ECHO);
    match tcsetattr(fd, TCSAFLUSH, &mut termios){
        Err(why) => panic!("Error while putting terminal in raw mode: {}", why),
        Ok(_) => {}
    }

}

fn main(){
    let mut c : char;
    let mut buff : [u8; 1] = [0];

    change_flags();

    loop {
        match stdin().read_exact(&mut buff){
            Ok(_) => {c = buff[0] as char},
            Err(_) => break
        }
        print!("{}", c);
    }
}