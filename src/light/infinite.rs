use std::f32::consts::{PI, FRAC_1_PI};
use std::path::Path;
use std::cmp::max;

use na::{Norm, Inverse, Origin, zero, origin};

use ::{Vector, Vector3f, Point2i, Point2f, Point3f, Transform};
use geometry::{spherical_phi, spherical_theta};
use interaction::{Interaction, SurfaceInteraction};
use light::{Light, LightFlags, VisibilityTester, INFINITE};
use mipmap::{MIPMap, WrapMode};
use ray::Ray;
use sampling::Distribution2D;
use spectrum::Spectrum;

pub struct InfiniteAreaLight {
    light_to_world: Transform,
    world_to_light: Transform,
    n_samples: u32,
    l_map: Box<MIPMap<Spectrum>>,
    world_center: Point3f,
    world_radius: f32,
    distribution: Box<Distribution2D>,
}

impl InfiniteAreaLight {
    pub fn new(l2w: Transform,
               n_samples: u32,
               power: Spectrum,
               texmap: &Path)
               -> InfiniteAreaLight {
        // Read texel data from texmap and initialise Lmap TODO
        let resolution = Point2i::new(1, 1);
        let l_map = Box::new(MIPMap::new(&resolution, &[power; 1], false, 0.0, WrapMode::Repeat));
        // initialize sampling PDFs for infinite area light TODO
        // - compute scalar-valued image img from environment map
        let (width, height) = (resolution.x as usize, resolution.y as usize);
        let filter = 1.0 / max(width, height) as f32;
        let mut img = Vec::with_capacity(width * height);
        for v in 0..height {
            let vp = v as f32 / height as f32;
            let sin_theta = (PI * (v as f32 + 0.5) / height as f32).sin();
            for u in 0..width {
                let up = u as f32 / width as f32;
                img[u + v * width] = l_map.lookup(&Point2f::new(up, vp), filter).y() * sin_theta;
            }
        }
        // - compute sampling distributions for rows and columns of image
        let distribution = Box::new(Distribution2D::new(&img[..], width, height));

        InfiniteAreaLight {
            light_to_world: l2w,
            world_to_light: l2w.inverse().unwrap(),
            n_samples: n_samples,
            l_map: l_map,
            world_center: origin(),
            world_radius: 100.0, // TODO
            distribution: distribution,
        }
    }
}

impl Light for InfiniteAreaLight {
    fn sample_li(&self,
                 isect: &SurfaceInteraction,
                 wo: &Vector,
                 u: &Point2f)
                 -> (Spectrum, Vector, f32, VisibilityTester) {
        // Find (u, v) sample coordinates in infinite light texture
        let (uv, map_pdf) = self.distribution.sample_continuous(u);
        if map_pdf == 0.0 {
            return (Spectrum::black(),
                    Vector3f::new(0.0, 0.0, 0.0),
                    0.0,
                    VisibilityTester::new(Interaction::from_point(&origin()),
                                          Interaction::from_point(&origin())));
        }
        // Convert infinite light sample point to direction
        let theta = uv[1] * PI;
        let phi = uv[0] * 2.0 * PI;
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();
        let wi = self.light_to_world *
                 Vector3f::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta);
        // Compute PDF for sampled infinite light direction
        let pdf = if sin_theta == 0.0 {
            0.0
        } else {
            map_pdf / (2.0 * PI * PI * sin_theta)
        };
        // Return radiance value for infinite light direction
        let target = isect.p + wi * (2.0 * self.world_radius);
        let vis = VisibilityTester::new(isect.into(), Interaction::from_point(&target));
        (self.l_map.lookup(&uv, 0.0), wi, pdf, vis)
    }

    fn pdf_li(&self, si: &SurfaceInteraction, w: &Vector) -> f32 {
        let wi = self.world_to_light * *w;
        let theta = spherical_theta(&wi);
        let phi = spherical_phi(&wi);
        let sin_theta = theta.sin();

        if sin_theta == 0.0 {
            0.0
        } else {
            self.distribution.pdf(&Point2f::new(phi * FRAC_1_PI * 0.5, theta * FRAC_1_PI)) /
            (2.0 * PI * PI * sin_theta)
        }
    }

    fn n_samples(&self) -> u32 {
        self.n_samples
    }

    fn flags(&self) -> LightFlags {
        INFINITE
    }

    fn power(&self) -> Spectrum {
        PI * self.world_radius * self.world_radius * self.l_map.lookup(&Point2f::new(0.5, 0.5), 0.5)
    }

    fn le(&self, ray: &Ray) -> Spectrum {
        let w = (self.world_to_light * ray.d).normalize();
        let st = Point2f::new(spherical_phi(&w) * FRAC_1_PI * 0.5,
                              spherical_theta(&w) * FRAC_1_PI);

        self.l_map.lookup(&st, 0.0)
    }
}