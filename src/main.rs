mod math;
mod engine;

fn main() {
    let a = math::Vec3::new(1.0,2.3,4.5);
    let mut b = math::Vec3::new(2.0,3.1,3.6);
    b.add(&a);
    b.z = 0.0;
    let mut c = a.project_on(&b);
    c.normalise();
    let mut d = a.clone();
    d.sub(&b);
    let e = b.cross(&d);

    let a = math::Mat3::identity();

    println!("Hello, {:?}", &e);
}