pub struct RingBell;

impl crossterm::Command for RingBell {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, "\x07")
    }
}

pub fn ring_bell() {
    crossterm::execute!(std::io::stdout(), RingBell).unwrap();
}
