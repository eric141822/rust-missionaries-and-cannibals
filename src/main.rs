fn main() {
    /* Get initial m_left and c_left from input */
    println!("Input number of missionaries and cannibals on the left side of the river: <missionaries> <cannibals>");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let m_left: i32 = iter.next().unwrap().parse().unwrap();
    let c_left: i32 = iter.next().unwrap().parse().unwrap();

    let s = rust_m_and_c::State {
        m_left,
        c_left,
        m_right: 0,
        c_right: 0,
        boat_pos: rust_m_and_c::Side::Left,
        parent: None,
    };
    /* Time the runtime */
    let now = std::time::Instant::now();
    let result = rust_m_and_c::solve(s);
    match result {
        Some(state) => {
            rust_m_and_c::path(&state);
        }
        None => {
            println!("No solution found!");
        }
    }
    println!("Time elapsed: {}us", now.elapsed().as_micros());
}
