#[derive(PartialEq)]
pub enum Side {
    Left,
    Right,
}
pub struct State {
    m_right: i32,
    m_left: i32,
    c_right: i32,
    c_left: i32,
    boat_pos: Side,
    parent: Option<Box<State>>,
}

impl State {
    pub fn is_finished(&self) -> bool {
        self.m_left == 0 && self.c_left == 0
    }
    pub fn is_valid(&self) -> bool {
        self.m_left >= 0
            && self.c_left >= 0
            && self.m_right >= 0
            && self.c_right >= 0
            && (self.m_right == 0 || self.m_right >= self.c_right)
            && (self.m_left == 0 || self.m_left >= self.c_left)
    }
    pub fn print(&self) {
        match self.boat_pos {
            Side::Left => {
                println!(
                    "M: {} C: {} | Boat: Left | M: {} C: {}",
                    self.m_left, self.c_left, self.m_right, self.c_right
                );
            }
            Side::Right => {
                println!(
                    "M: {} C: {} | Boat: Right | M: {} C: {}",
                    self.m_left, self.c_left, self.m_right, self.c_right
                );
            }
        }
    }
    pub fn path(&self) {
        match self.parent {
            Some(ref parent) => {
                parent.path();
                self.print();
            }
            None => {
                self.print();
            }
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
