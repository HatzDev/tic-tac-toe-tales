const PI: f32 = 3.141592653589793;

fn rotate_uvs(uv: vec2<f32>, angle: f32) -> vec2<f32> {
    var rotated_uvs: vec2<f32> = vec2<f32>(0.0, 0.0);
    let uv_center: vec2<f32> = vec2<f32>(0.5, 0.5);

    let theta: f32 = angle * (PI / 180.0);
    let cos_theta: f32 = cos(theta);
    let sin_theta: f32 = sin(theta);

    let u_diff: f32 = uv.x - uv_center.x;
    let v_diff: f32 = uv.y - uv_center.y;

    rotated_uvs.x = uv_center.x + u_diff * cos_theta - v_diff * sin_theta;
    rotated_uvs.y = uv_center.y + u_diff * sin_theta + v_diff * cos_theta;

    return rotated_uvs;
}

fn flip_uv_y(uvs: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(uvs.x, 1.0 - uvs.y);
}

fn flip_uv_x(uvs: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(1.0 - uvs.x, uvs.y);
}

fn tile(uvs: vec2<f32>, scaling_vector: vec2<f32>, time: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(fract(uvs.x * scaling_vector.x - time.x), fract(uvs.y * scaling_vector.y - time.y));
}