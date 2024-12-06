pub mod template;

// Use this file to add helper functions and additional modules.

pub mod directions {
    pub type Direction = (i32, i32);

    pub const UP: Direction = (-1, 0);
    pub const DOWN: Direction = (1, 0);
    pub const LEFT: Direction = (0, -1);
    pub const RIGHT: Direction = (0, 1);
    pub const UPLEFT: Direction = (-1, -1);
    pub const UPRIGHT: Direction = (-1, 1);
    pub const DOWNLEFT: Direction = (1, -1);
    pub const DOWNRIGHT: Direction = (1, 1);

    pub const DIRECTIONS: [(i32, i32); 8] =
        [UPLEFT, LEFT, DOWNLEFT, DOWN, DOWNRIGHT, RIGHT, UPRIGHT, UP];
}
