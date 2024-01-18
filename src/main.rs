use std::io::{self, Read};
use termios::*;
use libc::{iscntrl, atexit};


fn reset_flags(mut orig_term : Termios){
    match tcsetattr(0, TCSAFLUSH, &mut orig_term) {
        Err(e) => panic!("Error while reseting the flags : {}", e),
        Ok(_) => {}
    }
}

fn change_flags(){
    let fd = 0;
    let mut orig_termios = Termios::from_fd(fd).unwrap();
    let mut termios = Termios::from_fd(fd).unwrap();
    match tcgetattr(fd, &mut termios){
        Err(why) => {
            reset_flags(orig_termios);
            panic!("Error while putting terminal in raw mode: {}", why);
        }
        Ok(_) => {}
    }


    termios.c_lflag &= !(// local flags
        ECHO // disables showing inputing characters on screen 
        | ICANON // disables canonical mode to read byte-by-byte
        | ISIG // disables ctrl+c, and ctrl+z (and macOS ctrl+y)
        | IEXTEN //disables ctrl+v
    );

    termios.c_iflag &= !(// input flags
        IXON // disables data transmission control (ctrl+s and ctrl+q)
        | ICRNL // disables translating '\r' to '\n'
    );

    termios.c_oflag &= !(// output flags
        OPOST // disables
    );

    termios.c_cc[VMIN] = 0; // minimum amount of bytes before read can return
    termios.c_cc[VTIME] = 1; // amount of time without input after which read returns

    match tcsetattr(fd, TCSAFLUSH, &mut termios){
        Err(why) => {
            reset_flags(orig_termios);
            panic!("Error while putting terminal in raw mode: {}", why);
        }
        Ok(_) => {}
    }

}

fn main(){
    let mut STDIN: i32 = 0;
    let mut c : char;
    let mut orig_termios: Termios = Termios::from_fd(STDIN).unwrap();
    change_flags();
    loop {
        match io::stdin().lock().bytes().next() {
            //added error handling in loop 
            Some(Ok(byte)) => {
                c = byte as char;
                if c == 'q' {
                    break;
                }
                print!("{}", c);
            },
            Some(Err(e)) => {

                panic!("Error when executing reading loop: {}", e);
            }
            None => {}
        }
    }
    reset_flags(orig_termios);
}
