extern crate rt;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glam::Vec3A;
use rt::{AABB, Bounds, Ray};

fn bounds_hit(c: &mut Criterion) {
    let bounds = AABB::new(Vec3A::new(-1.0,-1.0,1.0), Vec3A::new(1.0,1.0,3.0));
    c.bench_function("AABB", |b| b.iter(|| bounds.intersects_bounds(black_box(Ray {position: Vec3A::zero(), direction : Vec3A::unit_z() }))));
}


fn bounds_miss(c: &mut Criterion) {
    let bounds = AABB::new(Vec3A::new(-1.0,-1.0,1.0), Vec3A::new(1.0,1.0,3.0));
    c.bench_function("AABB", |b| b.iter(|| bounds.intersects_bounds(black_box(Ray {position: Vec3A::zero(), direction : -Vec3A::unit_z() }))));
}

criterion_group!(benches, bounds_hit, bounds_miss);
criterion_main!(benches);