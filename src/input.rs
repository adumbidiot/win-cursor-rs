use winapi::um::winuser::{
    INPUT,
    INPUT_MOUSE,
    MOUSEEVENTF_ABSOLUTE,
    MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MOVE,
    MOUSEEVENTF_MOVE_NOCOALESCE,
    MOUSEEVENTF_RIGHTDOWN,
    MOUSEEVENTF_RIGHTUP,
};

/// An input
#[derive(Debug, Clone)]
pub enum Input {
    /// Mouse Input Event
    Mouse(MouseInput),
}

impl Input {
    /// Get a mut `MouseInput` if this is a `MouseInput`.
    pub fn get_mouse_input_mut(&mut self) -> Option<&mut MouseInput> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Mouse(input) = self {
            Some(input)
        } else {
            None
        }
    }
}

impl From<MouseInput> for Input {
    fn from(mouse_input: MouseInput) -> Self {
        Self::Mouse(mouse_input)
    }
}

/// Mouse Input Event
#[derive(Debug, Clone)]
pub struct MouseInput {
    /// X position or x delta, based on flags
    pub x: i32,

    /// Y position or y delta, based on flags
    pub y: i32,

    /// The mouse movement type
    pub move_type: MoveType,

    /// Whether mouse movement occured
    pub movement: bool,

    /// Whether to coalesce move messages
    pub coalesce: bool,

    /// The left button was pressed
    pub left_down: bool,

    /// The left button was released
    pub left_up: bool,

    /// The right button was pressed
    pub right_down: bool,

    /// The right button was released
    pub right_up: bool,

    /// The middle button was pressed
    pub middle_down: bool,

    /// The middle button was released
    pub middle_up: bool,
}

impl MouseInput {
    /// Make a new mouse input
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,

            move_type: Default::default(),
            movement: false,
            coalesce: true,

            left_down: false,
            left_up: false,
            right_down: false,
            right_up: false,
            middle_down: false,
            middle_up: false,
        }
    }
}

impl Default for MouseInput {
    fn default() -> Self {
        Self::new()
    }
}

/// The mouse movement type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveType {
    /// Absolute movement
    Absolute,

    /// Relative movement
    Relative,
}

impl Default for MoveType {
    fn default() -> Self {
        MoveType::Relative
    }
}

impl From<&'_ Input> for INPUT {
    fn from(input: &'_ Input) -> Self {
        match input {
            Input::Mouse(mouse_input) => {
                let input = unsafe {
                    let mut input: INPUT = std::mem::zeroed();
                    input.type_ = INPUT_MOUSE;
                    input.u.mi_mut().dx = mouse_input.x;
                    input.u.mi_mut().dy = mouse_input.y;

                    let mut flags = 0;

                    if mouse_input.move_type == MoveType::Absolute {
                        flags |= MOUSEEVENTF_ABSOLUTE;
                    }

                    if mouse_input.movement {
                        flags |= MOUSEEVENTF_MOVE;
                    }

                    if !mouse_input.coalesce {
                        flags |= MOUSEEVENTF_MOVE_NOCOALESCE;
                    }

                    if mouse_input.left_down {
                        flags |= MOUSEEVENTF_LEFTDOWN;
                    }

                    if mouse_input.left_up {
                        flags |= MOUSEEVENTF_LEFTUP;
                    }

                    if mouse_input.right_down {
                        flags |= MOUSEEVENTF_RIGHTDOWN;
                    }

                    if mouse_input.right_up {
                        flags |= MOUSEEVENTF_RIGHTUP;
                    }

                    input.u.mi_mut().dwFlags |= flags;

                    input
                };

                input
            }
        }
    }
}
