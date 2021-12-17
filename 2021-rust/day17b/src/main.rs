fn main() {
    let x_range = (269, 292);
    let y_range = (-68, -44);
    let mut result = 0;

    for x_init_vel in 1..1000 {
        for y_init_vel in -1000..1000 {
            let mut x = 0;
            let mut y = 0;
            let mut x_vel = x_init_vel;
            let mut y_vel = y_init_vel;

            loop {
                x += x_vel;
                if x_vel > 0 {
                    x_vel -= 1;
                }

                y += y_vel;
                y_vel -= 1;

                if x_range.0 <= x && x <= x_range.1 && y_range.0 <= y && y <= y_range.1 {
                    // we hit the target
                    result += 1;
                    break;
                }

                if x_range.1 < x || y < y_range.0 {
                    // we missed the target
                    break;
                }
            }
        }
    }

    println!("{}", result);
}
