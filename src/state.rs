#[derive(PartialEq, Clone)]
pub enum Side {
    Left,
    Right,
}
#[derive(Clone)]
pub struct State {
    pub m_right: i32,
    pub m_left: i32,
    pub c_right: i32,
    pub c_left: i32,
    pub boat_pos: Side,
    pub parent: Option<Box<State>>,
}

pub fn is_finished(state: &State) -> bool {
    state.m_left == 0 && state.c_left == 0
}

pub fn is_valid(state: &State) -> bool {
    state.m_left >= 0
        && state.c_left >= 0
        && state.m_right >= 0
        && state.c_right >= 0
        && (state.m_right == 0 || state.m_right >= state.c_right)
        && (state.m_left == 0 || state.m_left >= state.c_left)
}

pub fn print(state: &State) {
    match state.boat_pos {
        Side::Left => {
            println!(
                "M: {} C: {} | Boat: Left | M: {} C: {}",
                state.m_left, state.c_left, state.m_right, state.c_right
            );
        }
        Side::Right => {
            println!(
                "M: {} C: {} | Boat: Right | M: {} C: {}",
                state.m_left, state.c_left, state.m_right, state.c_right
            );
        }
    }
}

pub fn path(state: &State) {
    match state.parent {
        Some(ref parent) => {
            path(parent);
            print(state);
        }
        None => {
            print(state);
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.m_right == other.m_right
            && self.m_left == other.m_left
            && self.c_right == other.c_right
            && self.c_left == other.c_left
            && self.boat_pos == other.boat_pos
    }
}
