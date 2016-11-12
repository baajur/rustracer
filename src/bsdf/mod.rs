use std::mem;
use na::{self, Cross, Dot, zero, Norm, clamp};

use ::{Vector, Point2f};
use colour::Colourf;
use intersection::Intersection;
use interaction::SurfaceInteraction;

bitflags! {
    pub flags BxDFType: u32 {
        const BSDF_REFLECTION   = 0b_00000001,
        const BSDF_TRANSMISSION = 0b_00000010,
        const BSDF_DIFFUSE      = 0b_00000100,
        const BSDF_GLOSSY       = 0b_00001000,
        const BSDF_SPECULAR     = 0b_00010000,
    }
}

/// Represents the Bidirectional Scattering Distribution Function.
/// It represents the properties of a material at a given point.
#[derive(Copy, Clone)]
pub struct BSDF<'a> {
    /// Index of refraction of the surface
    eta: f32,
    /// Shading normal (i.e. potentially affected by bump-mapping)
    ns: Vector,
    /// Geometry normal
    ng: Vector,
    ss: Vector,
    ts: Vector,
    bxdfs: &'a [Box<BxDF + Sync + Send>],
}

impl<'a> BSDF<'a> {
    pub fn new(isect: &SurfaceInteraction,
               eta: f32,
               bxdfs: &'a [Box<BxDF + Sync + Send>])
               -> BSDF<'a> {
        let ss = isect.dpdu.normalize();
        BSDF {
            eta: eta,
            ns: isect.shading.n,
            ng: isect.n,
            ss: ss,
            ts: isect.shading.n.cross(&ss),
            bxdfs: bxdfs,
        }
    }

    /// Evaluate the BSDF for the given incoming light direction and outgoing light direction.
    pub fn f(&self, wi_w: &Vector, wo_w: &Vector, flags: BxDFType) -> Colourf {
        let wi = self.world_to_local(wi_w);
        let wo = self.world_to_local(wo_w);
        let reflect = wi_w.dot(&self.ng) * wo_w.dot(&self.ng) > 0.0;
        self.bxdfs
            .iter()
            .filter(|b| {
                // Make sure we only evaluate reflection or transmission based on whether wi and wo
                // lie in the same hemisphere.
                b.matches(flags) &&
                ((reflect && (b.get_type().contains(BSDF_REFLECTION))) ||
                 (!reflect && (b.get_type().contains(BSDF_TRANSMISSION))))
            })
            .fold(Colourf::black(), |c, b| c + b.f(&wi, &wo))
    }

    pub fn sample_f(&self,
                    wo_w: &Vector,
                    sample: (f32, f32),
                    flags: BxDFType)
                    -> (Colourf, Vector, f32) {
        if !flags.contains(BSDF_SPECULAR) {
            unimplemented!();
        }

        if flags.contains(BSDF_REFLECTION) {
            let wo = self.world_to_local(&wo_w);
            let wi = Vector::new(-wo.x, -wo.y, wo.z);
            let cos_theta_i = wi.z;
            let kr = fr_dielectric(cos_theta_i, 1.0, self.eta);
            let colour = Colourf::rgb(1.0, 1.0, 1.0) * kr / cos_theta_i.abs();

            assert!(!colour.has_nan());
            return (colour, self.local_to_world(&wi), 1.0);
        } else if flags.contains(BSDF_TRANSMISSION) {
            let wo = self.world_to_local(&wo_w);
            let entering = wo.z > 0.0;
            let (eta_i, eta_t) = if entering {
                (1.0, self.eta)
            } else {
                (self.eta, 1.0)
            };
            let n = if wo.z < 0.0 {
                -Vector::z()
            } else {
                Vector::z()
            };
            return refract(&wo, &n, eta_i / eta_t)
                .map(|wi| {
                    let cos_theta_i = wi.z;
                    let kr = fr_dielectric(cos_theta_i, 1.0, self.eta);
                    let colour = Colourf::rgb(1.0, 1.0, 1.0) * (1.0 - kr) / cos_theta_i.abs();

                    assert!(!colour.has_nan());
                    (colour, self.local_to_world(&wi), 1.0)
                })
                .unwrap_or((Colourf::black(), zero(), 0.0));
        }

        (Colourf::black(), zero(), 0.0)
    }

    fn world_to_local(&self, v: &Vector) -> Vector {
        Vector::new(v.dot(&self.ss), v.dot(&self.ts), v.dot(&self.ns))
    }

    fn local_to_world(&self, v: &Vector) -> Vector {
        Vector::new(self.ss.x * v.x + self.ts.x * v.y + self.ns.x * v.z,
                    self.ss.y * v.x + self.ts.y * v.y + self.ns.y * v.z,
                    self.ss.z * v.z + self.ts.z * v.y + self.ns.z * v.z)
    }
}

// Common geometric functions
#[inline]
fn cos_theta(w: &Vector) -> f32 {
    w.z
}

#[inline]
fn cos2_theta(w: &Vector) -> f32 {
    w.z * w.z
}

#[inline]
fn abs_cos_theta(w: &Vector) -> f32 {
    w.z.abs()
}

#[inline]
fn sin2_theta(w: &Vector) -> f32 {
    (1.0 - cos2_theta(w)).max(0.0)
}

#[inline]
fn sin_theta(w: &Vector) -> f32 {
    sin2_theta(w).sqrt()
}

#[inline]
fn tan_theta(w: &Vector) -> f32 {
    sin_theta(w) / cos_theta(w)
}

#[inline]
fn tan2_theta(w: &Vector) -> f32 {
    sin2_theta(w) / cos2_theta(w)
}

#[inline]
fn cos_phi(w: &Vector) -> f32 {
    let sin_theta = sin_theta(w);
    if sin_theta == 0.0 {
        0.0
    } else {
        na::clamp(w.x / sin_theta, -1.0, 1.0)
    }
}

#[inline]
fn sin_phi(w: &Vector) -> f32 {
    let sin_theta = sin_theta(w);
    if sin_theta == 0.0 {
        0.0
    } else {
        na::clamp(w.y / sin_theta, -1.0, 1.0)
    }
}

#[inline]
fn cos2_phi(w: &Vector) -> f32 {
    cos_phi(w) / cos_phi(w)
}

#[inline]
fn sin2_phi(w: &Vector) -> f32 {
    sin_phi(w) / sin_phi(w)
}

#[inline]
fn cos_d_phi(wa: &Vector, wb: &Vector) -> f32 {
    na::clamp((wa.x * wb.x + wa.y * wa.y) /
              ((wa.x * wa.x + wa.y * wa.y) * (wb.x * wb.x + wb.y * wb.y)).sqrt(),
              -1.0,
              1.0)
}

pub trait BxDF {
    fn f(&self, wo: &Vector, wi: &Vector) -> Colourf;
    fn sample_f(&self, wo: &Vector, sample: &Point2f) -> (Vector, f32, Option<BxDFType>, Colourf);
    // fn rho(&self, wo: &Vector, n_samples: u32) -> (Point2f, Colourf);
    // fn rho_hh(&self, n_samples: u32) -> (Point2f, Point2f, Colourf);
    fn matches(&self, flags: BxDFType) -> bool {
        self.get_type() & flags == self.get_type()
    }

    fn get_type(&self) -> BxDFType;
}

pub struct ScaledBxDF {
    bxdf: Box<BxDF>,
    scale: Colourf,
}

impl ScaledBxDF {
    fn new(bxdf: Box<BxDF>, scale: Colourf) -> ScaledBxDF {
        ScaledBxDF {
            bxdf: bxdf,
            scale: scale,
        }
    }
}

impl BxDF for ScaledBxDF {
    fn f(&self, wo: &Vector, wi: &Vector) -> Colourf {
        self.bxdf.f(wo, wi) * self.scale
    }
    fn sample_f(&self, wo: &Vector, sample: &Point2f) -> (Vector, f32, Option<BxDFType>, Colourf) {
        let (wi, pdf, bxdftype, spectrum) = self.bxdf.sample_f(wo, sample);
        (wi, pdf, bxdftype, spectrum * self.scale)
    }
    // fn rho(&self, wo: &Vector, n_samples: u32) -> (Point2f, Colourf) {
    //     let (sample, spectrum) = self.bxdf.rho(wo, n_samples);
    //     (sample, spectrum * self.scale)
    // }
    // fn rho_hh(&self, n_samples: u32) -> (Point2f, Point2f, Colourf) {
    //     let (sample1, sample2, spectrum) = self.bxdf.rho_hh(n_samples);
    //     (sample1, sample2, spectrum * self.scale)
    // }
    fn get_type(&self) -> BxDFType {
        self.bxdf.get_type()
    }
}

/// Compute the reflection direction
fn reflect(wo: &Vector, n: &Vector) -> Vector {
    (-(*wo) + *n * 2.0 * wo.dot(n)).normalize()
}

/// Compute the refraction direction
fn refract(i: &Vector, n: &Vector, eta: f32) -> Option<Vector> {
    let cos_theta_i = n.dot(i);
    let sin2theta_i = (1.0 - cos_theta_i * cos_theta_i).max(0.0);
    let sin2theta_t = eta * eta * sin2theta_i;

    if sin2theta_t >= 1.0 {
        None
    } else {
        let cos_theta_t = (1.0 - sin2theta_t).sqrt();
        Some(eta * -*i + (eta * cos_theta_i - cos_theta_t) * *n)
    }
}

/// Compute the Fresnel coefficient for dielectric materials
fn fr_dielectric(cos_theta_i: f32, eta_i: f32, eta_t: f32) -> f32 {
    let mut cos_theta_i = clamp(cos_theta_i, -1.0, 1.0);
    let (mut eta_i, mut eta_t) = (eta_i, eta_t);
    if cos_theta_i <= 0.0 {
        // If leaving the surface, swap the indices of refraction
        mem::swap(&mut eta_i, &mut eta_t);
        cos_theta_i = cos_theta_i.abs();
    }

    let sin_theta_i = (1.0 - cos_theta_i * cos_theta_i).max(0.0).sqrt();
    let sin_theta_t = eta_i / eta_t * sin_theta_i;
    if sin_theta_t >= 1.0 {
        // Total internal reflection
        1.0
    } else {
        let cos_theta_t = (1.0 - sin_theta_t * sin_theta_t).max(0.0).sqrt();
        // Reflectance for parallel polarized light
        let r_parl = ((eta_t * cos_theta_i) - (eta_i * cos_theta_t)) /
                     ((eta_t * cos_theta_i) + (eta_i * cos_theta_t));
        // Reflectance for perpendicular polarized light
        let r_perp = ((eta_i * cos_theta_i) - (eta_t * cos_theta_t)) /
                     ((eta_i * cos_theta_i) + (eta_t * cos_theta_t));
        // Total reflectance for unpolarized light
        0.5 * (r_parl * r_parl + r_perp * r_perp)
    }
}

fn fr_conductor(cos_theta_i: f32, eta_i: &Colourf, eta_t: &Colourf, k: &Colourf) -> Colourf {
    let mut cos_theta_i = clamp(cos_theta_i, -1.0, 1.0);
    let eta = *eta_t / *eta_i;
    let eta_k = *k / *eta_i;

    let cos2_theta_i = cos_theta_i * cos_theta_i;
    let sin2_theta_i = 1.0 - cos2_theta_i;
    let eta2 = eta * eta;
    let eta_k2 = eta_k * eta_k;

    let t0 = eta2 - eta_k2 - sin2_theta_i;
    let a2plusb2 = (t0 * t0 + 4.0 * eta2 * eta_k2).sqrt();
    let t1 = a2plusb2 + cos2_theta_i;
    let a = (0.5 * (a2plusb2 + t0)).sqrt();
    let t2 = 2.0 * cos_theta_i * a;
    let r_s = (t1 - t2) / (t1 + t2);

    let t3 = cos2_theta_i * a2plusb2 + sin2_theta_i * sin2_theta_i;
    let t4 = t2 * sin2_theta_i;
    let r_p = r_s * (t3 - t4) / (t3 + t4);

    0.5 * (r_p + r_s)
}

/// Trait for Fresnel materials
pub trait Fresnel {
    fn evaluate(&self, cos_theta_i: f32) -> Colourf;
}

/// Fresnel for conductor materials
pub struct FresnelConductor {
    eta_i: Colourf,
    eta_t: Colourf,
    k: Colourf,
}

impl FresnelConductor {
    pub fn new(eta_i: Colourf, eta_t: Colourf, k: Colourf) -> FresnelConductor {
        FresnelConductor {
            eta_i: eta_i,
            eta_t: eta_t,
            k: k,
        }
    }
}

impl Fresnel for FresnelConductor {
    fn evaluate(&self, cos_theta_i: f32) -> Colourf {
        fr_conductor(cos_theta_i.abs(), &self.eta_i, &self.eta_t, &self.k)
    }
}

/// Fresnel for dielectric materials
pub struct FresnelDielectric {
    eta_i: f32,
    eta_t: f32,
}

impl Fresnel for FresnelDielectric {
    fn evaluate(&self, cos_theta_i: f32) -> Colourf {
        Colourf::grey(fr_dielectric(cos_theta_i.abs(), self.eta_i, self.eta_t))
    }
}

/// BRDF for perfect specular reflection
pub struct SpecularReflection {
    r: Colourf,
    fresnel: Box<Fresnel + Send + Sync>,
}

impl SpecularReflection {
    pub fn new(r: Colourf, fresnel: Box<Fresnel + Send + Sync>) -> SpecularReflection {
        SpecularReflection {
            r: r,
            fresnel: fresnel,
        }
    }
}

impl BxDF for SpecularReflection {
    fn f(&self, wo: &Vector, wi: &Vector) -> Colourf {
        // The probability to call f() with the exact (wo, wi) for specular reflection is 0, so we
        // return black here. Use sample_f() instead.
        Colourf::black()
    }

    fn sample_f(&self, wo: &Vector, _sample: &Point2f) -> (Vector, f32, Option<BxDFType>, Colourf) {
        // There's only one possible wi for a given wo, so we always return it with a pdf of 1.
        let wi = Vector::new(-wo.x, -wo.y, wo.z);
        let spectrum = self.fresnel.evaluate(cos_theta(&wi)) * self.r / abs_cos_theta(&wi);
        (wi, 1.0, None, spectrum)
    }

    fn get_type(&self) -> BxDFType {
        BSDF_SPECULAR | BSDF_REFLECTION
    }
}
