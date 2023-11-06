mod state;

use std::collections::VecDeque;

pub use state::Side;
pub use state::State;

pub fn check_and_add(v: &mut Vec<State>, s: State) {
    if s.is_valid() {
        v.push(s);
    }
}

pub fn contains(v: &Vec<State>, s: &State) -> bool {
    for i in v {
        if i == s {
            return true;
        }
    }
    false
}

pub fn generate_state_vec(s: &State) -> Vec<State> {
    let mut v: Vec<State> = Vec::new();
    match s.boat_pos {
        Side::Left => {
            check_and_add(
                &mut v,
                State {
                    m_left: s.m_left - 1,
                    c_left: s.c_left - 1,
                    m_right: s.m_right + 1,
                    c_right: s.c_right + 1,
                    boat_pos: Side::Right,
                    parent: Some(Box::new(s.clone())),
                },
            );
            check_and_add(&mut v, {
                State {
                    m_left: s.m_left - 2,
                    c_left: s.c_left,
                    m_right: s.m_right + 2,
                    c_right: s.c_right,
                    boat_pos: Side::Right,
                    parent: Some(Box::new(s.clone())),
                }
            });
            check_and_add(&mut v, {
                State {
                    m_left: s.m_left,
                    c_left: s.c_left - 2,
                    m_right: s.m_right,
                    c_right: s.c_right + 2,
                    boat_pos: Side::Right,
                    parent: Some(Box::new(s.clone())),
                }
            });
            check_and_add(&mut v, {
                State {
                    m_left: s.m_left - 1,
                    c_left: s.c_left,
                    m_right: s.m_right + 1,
                    c_right: s.c_right,
                    boat_pos: Side::Right,
                    parent: Some(Box::new(s.clone())),
                }
            });
            check_and_add(&mut v, {
                State {
                    m_left: s.m_left,
                    c_left: s.c_left - 1,
                    m_right: s.m_right,
                    c_right: s.c_right + 1,
                    boat_pos: Side::Right,
                    parent: Some(Box::new(s.clone())),
                }
            });
        }
        Side::Right => {
            check_and_add(&mut v, {
                State {
                    m_left: s.m_left + 1,
                    c_left: s.c_left + 1,
                    m_right: s.m_right - 1,
                    c_right: s.c_right - 1,
                    boat_pos: Side::Left,
                    parent: Some(Box::new(s.clone())),
                }
            });
            check_and_add(&mut v, {
                State {
                    m_left: s.m_left + 2,
                    c_left: s.c_left,
                    m_right: s.m_right - 2,
                    c_right: s.c_right,
                    boat_pos: Side::Left,
                    parent: Some(Box::new(s.clone())),
                }
            });
            check_and_add(&mut v, {
                State {
                    m_left: s.m_left,
                    c_left: s.c_left + 2,
                    m_right: s.m_right,
                    c_right: s.c_right - 2,
                    boat_pos: Side::Left,
                    parent: Some(Box::new(s.clone())),
                }
            });
            check_and_add(&mut v, {
                State {
                    m_left: s.m_left + 1,
                    c_left: s.c_left,
                    m_right: s.m_right - 1,
                    c_right: s.c_right,
                    boat_pos: Side::Left,
                    parent: Some(Box::new(s.clone())),
                }
            });
            check_and_add(&mut v, {
                State {
                    m_left: s.m_left,
                    c_left: s.c_left + 1,
                    m_right: s.m_right,
                    c_right: s.c_right - 1,
                    boat_pos: Side::Left,
                    parent: Some(Box::new(s.clone())),
                }
            });
        }
    }
    v
}

pub fn solve(s: State) -> Option<State> {
    let mut v: VecDeque<State> = VecDeque::new();
    let mut visited: Vec<State> = Vec::new();
    v.push_back(s);
    while !v.is_empty() {
        let s = v.pop_front().unwrap();
        if s.is_finished() {
            return Some(s);
        }
        let new_states = generate_state_vec(&s);
        for i in &new_states {
            if !contains(&visited, i) {
                v.push_back(i.clone());
                visited.push(i.clone());
            }
        }
    }
    None
}
