mod input;

pub use crate::input::{
    Input,
    MouseInput,
};
use std::mem::MaybeUninit;
use winapi::{
    shared::windef::RECT,
    um::winuser::{
        GetClipCursor,
        GetCursorPos,
        SendInput,
        SetCursorPos,
        INPUT,
    },
};

/// Set the cursor position
pub fn set_position(point: (i32, i32)) -> Result<(), std::io::Error> {
    let ret = unsafe { SetCursorPos(point.0, point.1) };

    if ret != 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

/// Get cursor position
pub fn get_positon() -> Result<(i32, i32), std::io::Error> {
    let mut point = MaybeUninit::zeroed();
    let ret = unsafe { GetCursorPos(point.as_mut_ptr()) };

    if ret != 0 {
        let point = unsafe { point.assume_init() };
        Ok((point.x, point.y))
    } else {
        Err(std::io::Error::last_os_error())
    }
}

/// Left Click
pub fn left_click() -> Result<(), std::io::Error> {
    let mut mouse_input = MouseInput::new();
    mouse_input.left_down = true;

    let mut inputs = vec![mouse_input.into()];
    send_inputs(&inputs)?;

    {
        let mouse_input = inputs[0].get_mouse_input_mut().expect("MouseInput");
        mouse_input.left_down = false;
        mouse_input.left_up = true;
    }

    send_inputs(&inputs)?;

    Ok(())
}

/// Right Click
pub fn right_click() -> Result<(), std::io::Error> {
    let mut mouse_input = MouseInput::new();
    mouse_input.right_down = true;

    let mut inputs = vec![mouse_input.into()];
    send_inputs(&inputs)?;

    {
        let mouse_input = inputs[0].get_mouse_input_mut().expect("MouseInput");
        mouse_input.right_down = false;
        mouse_input.right_up = true;
    }

    send_inputs(&inputs)?;

    Ok(())
}

/// Get cursor boundaries
pub fn get_bounds() -> Result<Rect, std::io::Error> {
    let mut rect = MaybeUninit::zeroed();
    let ret = unsafe { GetClipCursor(rect.as_mut_ptr()) };

    if ret != 0 {
        let rect = unsafe { rect.assume_init() };
        Ok(rect.into())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

/// Send mouse, keyboard, and hardware inputs
pub fn send_inputs(inputs: &[Input]) -> Result<usize, std::io::Error> {
    if inputs.is_empty() {
        return Ok(0);
    }

    let mut inputs: Vec<INPUT> = inputs.iter().map(Into::into).collect();

    let num_processed = unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_mut_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        )
    };

    if num_processed == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(num_processed as usize)
    }
}

/// A Rectangle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    /// Make a new Rect
    pub fn new_xywh(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    /// Get the top-left point
    pub fn top_left(self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Get the bottom-right point
    pub fn bottom_right(self) -> (i32, i32) {
        (self.x + self.width, self.y + self.height)
    }
}

impl From<RECT> for Rect {
    fn from(rect: RECT) -> Self {
        Rect {
            x: rect.left,
            y: rect.top,
            width: rect.right - rect.left,
            height: rect.bottom - rect.top,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        let point = (0, 0);
        crate::set_position(point).unwrap();
        let pos = crate::get_positon().unwrap();
        assert_eq!(pos, point);

        crate::left_click().unwrap();
        crate::right_click().unwrap();

        let bounds = crate::get_bounds().unwrap();
        dbg!(&bounds);
        crate::set_position(bounds.bottom_right()).unwrap();
    }
}
