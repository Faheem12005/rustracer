use std::sync::atomic::{AtomicUsize, Ordering};

pub struct RenderStats {
    pub rays_cast: AtomicUsize,
    pub intersection_tests: AtomicUsize,
    pub bounces: AtomicUsize,
    pub rays_missed: AtomicUsize,
}

impl RenderStats {
    pub fn inc_rays_cast(&self) {
        self.rays_cast.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_intersection_tests(&self) {
        self.intersection_tests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_bounces(&self) {
        self.bounces.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_ray_missed(&self) {
        self.rays_missed.fetch_add(1, Ordering::Relaxed);
    }
    pub fn report(&self) {
        let rays_cast = self.rays_cast.load(Ordering::Relaxed);
        let intersection_tests = self.intersection_tests.load(Ordering::Relaxed);
        let bounces = self.bounces.load(Ordering::Relaxed);
        let rays_missed = self.rays_missed.load(Ordering::Relaxed);

        eprintln!("Rays cast:           {:?}", rays_cast);
        eprintln!("Intersection tests:  {:?}", intersection_tests);
        eprintln!("Bounces:             {:?}", bounces);
        eprintln!("Rays missed:         {:?}", rays_missed);

        if rays_cast > 0 {
            eprintln!(
                "Tests per ray:       {:.1}",
                intersection_tests as f64 / rays_cast as f64
            );
        }
    }
}