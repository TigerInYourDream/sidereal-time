use std::f64::consts::PI;
use std::f64::consts::E;

pub fn to_radians(x: f64) -> f64 {
    x * PI / 180f64
}

pub fn to_angle(x: f64) -> f64 {
    x * 180f64 / PI
}

pub fn houses_system() {
    println!("houses_system!");
    let a = to_radians(1.95f64).tan() / to_radians(23.45f64).cos();
    let mc = a.atan();
    println!("{:?}", to_angle(mc));

    //ARCTAN(- ( (TAN f x SIN e) + (SIN RAMC x COS e) ) ÷ COS RAMC)
    let f = -to_radians(52.21f64).tan() * E.sin() + to_radians(1.95f64).sin() * E.cos();
    println!("{:?}", f);
    let mut asc = to_angle(f.atan());
    println!("asc {:?}", asc);
    if asc < 0f64 {
        asc = 90f64 - asc
    }
    println!("asc {:?}", asc);
}

#[cfg(test)]
mod tests {
    use crate::house_system::houses_system;

    #[test]
    pub fn houses_system_test() {
        houses_system()
    }
}