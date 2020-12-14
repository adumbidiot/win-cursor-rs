# win-cursor-rs
An small, incomplete API to control cursors on Windows for personal amusement. 

## Example

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
```

## References
 * https://stackoverflow.com/questions/13807543/windows-command-script-that-moves-mouse-cursor-n-pixels/53769159
 * https://github.com/npocmaka/batch.scripts/blob/dc744016d2e34342770e77c23e9dc70303b7e3aa/hybrids/.net/c/mouse.bat