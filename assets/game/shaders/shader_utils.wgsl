const PI: f32 = 3.141592653589793;

fn rotateUVs(uv: vec2<f32>, angle: f32) -> vec2<f32> {
    var rotatedUvs: vec2<f32> = vec2<f32>(0.0, 0.0);
    let uv_center: vec2<f32> = vec2<f32>(0.5, 0.5);

    let theta: f32 = angle * (PI / 180.0);
    let cos_theta: f32 = cos(theta);
    let sin_theta: f32 = sin(theta);

    let u_diff: f32 = uv.x - uv_center.x;
    let v_diff: f32 = uv.y - uv_center.y;

    rotatedUvs.x = uv_center.x + u_diff * cos_theta - v_diff * sin_theta;
    rotatedUvs.y = uv_center.y + u_diff * sin_theta + v_diff * cos_theta;

    return rotatedUvs;
}

fn flipUVsY(uv: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(uv.x, 1.0 - uv.y);
}

fn flipUVsX(uv: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(1.0 - uv.x, uv.y);
}