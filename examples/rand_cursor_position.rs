#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;

fn main() {
    loop {
        if let Ok(bounds) = win_cursor::get_bounds() {
            let min = bounds.top_left();
            let max = bounds.bottom_right();

            let mut rng = rand::thread_rng();
            let x = rng.gen_range(min.0..max.0);
            let y = rng.gen_range(min.1..max.1);

            let _ = win_cursor::set_position((x, y)).is_ok();
        }
    }
}
