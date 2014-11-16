pub fn add_three_times_four(x: int) -> int {
    times_four(add_three(x))
}

fn add_three(num: int) -> int { num + 3i }
fn times_four(num: int) -> int { num * 4i }
