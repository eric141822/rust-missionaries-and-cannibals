use rust_m_and_c::State;
use rust_m_and_c::Side;

fn main() {
    let s = State {
        m_left: 3,
        c_left: 3,
        m_right: 0,
        c_right: 0,
        boat_pos: Side::Left,
        parent: None,
    };
    /* Time the runtime */
    let now = std::time::Instant::now();
    let result = rust_m_and_c::solve(s);
    match result {
        Some(state) => {
            state.path();
        }
        None => {
            println!("No solution found!");
        }
    }
    println!("Time elapsed: {}us", now.elapsed().as_micros());
}
