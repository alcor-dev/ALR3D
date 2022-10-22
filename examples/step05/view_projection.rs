use std::f32::consts::FRAC_PI_6;
use cgmath::*;

fn main(){
    let eye: Point3<f32> = Point3::new(3.0, 4.0, 5.0);
    let center: Point3<f32> = Point3::new(-3.0, -4.0, -5.0);
    let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

    let view_mat = Matrix4::look_at_rh(eye, center, up);

    println!("\nPosición del espectador: {:?}", eye);
    println!("Punto al que el espectador está mirando: {:?}", center);
    println!("Dirección de up: {:?}", up);
    println!("Matriz de vista: {:?}", view_mat);

    let left = -3.0;
    let right = 3.0;
    let bottom = -5.0;
    let top = 5.0;
    let near = 1.0;
    let far = 100.0;
    let fovy = FRAC_PI_6;
    let aspect = 1.5;

    let frustum_mat = frustum(left, right, bottom, top, near, far);
    let persp_mat = perspective(Rad(fovy), aspect, near, far);

    println!("\nMatriz Frustum: {:?}\n", frustum_mat);
    println!("Matriz perspectiva: {:?}\n", persp_mat);

    let mut ortho_mat = [0.0; 16];
    let ortho_mat = ortho(left, right, bottom, top, near, far);
    println!("Matriz ortográfica: {:?}", ortho_mat);
}