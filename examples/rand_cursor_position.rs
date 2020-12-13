use rand::Rng;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    loop {
        let bounds = win_cursor::get_bounds()?;
        let min = bounds.top_left();
        let max = bounds.bottom_right();

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(min.0, max.0);
        let y = rng.gen_range(min.1, max.1);

        win_cursor::set_position((x, y))?;

        std::thread::sleep(Duration::from_millis(10));
    }
}
