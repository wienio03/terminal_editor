use std::io::{stdin, Read};
use termios::*;
use libc::{iscntrl, atexit};
use std::rc::Rc;

static ORIGINAL_FLAGS : Rc<Termios> = Rc::new(std::mem::size_of::<Termios>);

fn reset_flags(){


}

fn set_flags(){
    let fd = 0;
    let mut termios = Termios::from_fd(fd).unwrap();
    match tcgetattr(fd, &mut termios){
        Err(why) => panic!("Error while putting terminal in raw mode: {}", why),
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
        Err(why) => panic!("Error while putting terminal in raw mode: {}", why),
        Ok(_) => {}
    }

}

fn main(){
    let mut c : char;
    let mut buff : [u8; 1] = [0];

    set_flags();

    loop {
        match stdin().read_exact(&mut buff){
            Ok(_) => {c = buff[0] as char},
            Err(_) => break
        }
        if c == 'q'{
            break;
        }
        print!("{}", c);
    }
}
