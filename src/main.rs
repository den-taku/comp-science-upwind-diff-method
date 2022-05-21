use std::f64::consts::PI;

fn main() {
    let delta_t = 1e-2;
    let delta_x = 1e-2;
    let lambda = delta_t / delta_x;

    let mut values = Vec::new();
    let mut u_0 = Vec::new();
    for x in 0..=100 {
        u_0.push((PI * delta_x * x as f64).sin());
    }
    values.push(u_0);
    for _ in 1..=100 {
        let mut u_t = vec![0.0; 101];
        // x = 0
        u_t[0] =
            lambda * values[values.len() - 1][1] + (1.0 - lambda) * values[values.len() - 1][0];
        for x in 1..100 {
            u_t[x] = lambda / 2.0 * values[values.len() - 1][x + 1] + values[values.len() - 1][x]
                - lambda / 2.0 * values[values.len() - 1][x - 1];
        }
        // x = 1
        u_t[100] = u_t[0];

        values.push(u_t);
    }

    // t = T (=1)
    let mut u_r = vec![0.0; 101];
    for x in 0..=100 {
        u_r[x] = (PI * (1.0 + delta_x * x as f64)).sin();
    }

    for (x, (_r, c)) in u_r.iter().zip(values[100].iter()).enumerate() {
        println!("{x:2}: {c}");
    }
}
