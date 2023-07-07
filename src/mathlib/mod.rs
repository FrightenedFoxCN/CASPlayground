pub fn _is_inside_epsilon_about(target: f64, res: f64, eps: f64) -> bool {
    return (target - res).abs() < eps;
}