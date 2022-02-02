extern crate rt;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glam::Vec3A;
use rt::{Bounds, Ray, Shape, Sphere, AABB};

fn bounds_hit(c: &mut Criterion) {
    let bounds = AABB::new(Vec3A::new(-1.0, -1.0, 1.0), Vec3A::new(1.0, 1.0, 3.0));
    c.bench_function("AABB Intersection (hit)", |b| {
        b.iter(|| {
            bounds.intersects_bounds(black_box(Ray {
                position: Vec3A::ZERO,
                direction: Vec3A::Z,
            }))
        })
    });
}

fn bounds_miss(c: &mut Criterion) {
    let bounds = AABB::new(Vec3A::new(-1.0, -1.0, 1.0), Vec3A::new(1.0, 1.0, 3.0));
    c.bench_function("AABB Intersection (miss)", |b| {
        b.iter(|| {
            bounds.intersects_bounds(black_box(Ray {
                position: Vec3A::new(10.0, 10.0, -1.0),
                direction: Vec3A::Z,
            }))
        })
    });
}

fn sphere_hit(c: &mut Criterion) {
    let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 3.0), 1.0);
    c.bench_function("Sphere Intersection (hit)", |b| {
        b.iter(|| {
            sphere.intersects(black_box(Ray {
                position: Vec3A::ZERO,
                direction: Vec3A::Z,
            }))
        })
    });
}

fn sphere_miss(c: &mut Criterion) {
    let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 3.0), 1.0);
    c.bench_function("Sphere Intersection (miss)", |b| {
        b.iter(|| {
            sphere.intersects(black_box(Ray {
                position: Vec3A::new(10.0, 10.0, -1.0),
                direction: Vec3A::Z,
            }))
        })
    });
}

criterion_group!(benches, bounds_hit, bounds_miss, sphere_hit, sphere_miss);
criterion_main!(benches);
