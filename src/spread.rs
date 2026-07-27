/// Deterministic cone spread using a random seed + pellet index.
///
/// All peers (server, local client, remote clients) produce identical pellet
/// directions given the same seed, aim direction, spread angle and index.
pub fn apply_spread_seeded(dir: &[f32; 3], spread_rad: f32, seed: u64, index: u32) -> [f32; 3] {
    if spread_rad <= 0.0 {
        return *dir;
    }

    let mix = seed as f32 * 1.618034 + index as f32 * 3.141593;
    let theta = (mix * 6.283185).fract() * 6.283185;
    let r = (mix.fract() * 1.618034).fract().sqrt();
    let phi = r * spread_rad;

    let (sin_t, cos_t) = theta.sin_cos();
    let (sin_p, cos_p) = phi.sin_cos();

    let up = [0.0, 1.0, 0.0];
    let right = cross(dir, &up);
    let right_len = (right[0] * right[0] + right[1] * right[1] + right[2] * right[2]).sqrt();
    let (right, up_local) = if right_len > 0.001 {
        let r = [right[0] / right_len, right[1] / right_len, right[2] / right_len];
        let u = cross(&r, dir);
        (r, u)
    } else {
        ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0])
    };

    let spread_dir = [
        dir[0] * cos_p + (right[0] * cos_t + up_local[0] * sin_t) * sin_p,
        dir[1] * cos_p + (right[1] * cos_t + up_local[1] * sin_t) * sin_p,
        dir[2] * cos_p + (right[2] * cos_t + up_local[2] * sin_t) * sin_p,
    ];

    let len = (spread_dir[0] * spread_dir[0] + spread_dir[1] * spread_dir[1] + spread_dir[2] * spread_dir[2]).sqrt();
    if len > 0.001 {
        [spread_dir[0] / len, spread_dir[1] / len, spread_dir[2] / len]
    } else {
        *dir
    }
}

fn cross(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
