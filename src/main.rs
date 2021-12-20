use ansi_term::Colour::Green;
use ansi_term::Colour::Red;
use ansi_term::Colour::Yellow;
use std::io::stdout;
use std::io::Write;
use std::thread;
use std::time;

fn main() {
    for i in 0..5000 {
        print!(
            "{}, {} {} ",
            Green.paint("Happy"),
            Red.paint("Holidays"),
            Yellow.paint("TrendNation!")
        );
        stdout().flush();
        thread::sleep(time::Duration::from_millis(80));
    }
}
