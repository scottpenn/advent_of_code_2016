mod days;

const MAX_DAY: u8 = 1;

fn main() {
    for day in 1..=MAX_DAY {
        days::run(day)
    }
}
