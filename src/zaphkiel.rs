#[macro_export]
macro_rules! exponential_decay {
    (current: $a: expr, target: $b: expr, decay: $decay: expr, delta: $delta: expr$(,)?) => {{
        let a = $a;
        let b = $b;
        let delta = $delta;
        let decay = 16.0;

        b + (a - b) * f32::exp(-decay * delta)
    }};
    (current: $a: expr, target: $b: expr, delta: $delta: expr$(,)?) => {{
        exponential_decay!(current: $a, target: $b, decay: 16.0, delta: $delta)
    }};
}
