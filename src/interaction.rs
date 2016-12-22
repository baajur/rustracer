use std::sync::Arc;
use ::{Point, Point2f, Vector, Transform};
use bsdf::BSDF;
use spectrum::Spectrum;
use primitive::Primitive;
use ray::Ray;
use shapes::Shape;
use transform;
use na::{self, Dot, Cross, Norm};
use material::TransportMode;
use fp::Ieee754;


// TODO Find a better design for this mess of inheritance...

pub struct Interaction {
    /// The point where the ray hit the primitive
    pub p: Point,
    /// Error bound for the intersection point
    pub p_error: Vector,
    /// Outgoing direction of the light at the intersection point (usually `-ray.d`)
    pub wo: Vector,
    /// Normal
    pub n: Vector,
}

impl Interaction {
    pub fn new(p: Point, p_error: Vector, wo: Vector, n: Vector) -> Interaction {
        Interaction {
            p: p,
            p_error: p_error,
            wo: wo,
            n: n,
        }
    }

    pub fn spawn_ray(&self, dir: &Vector) -> Ray {
        assert!(dir.x != 0.0 && dir.y != 0.0 && dir.z != 0.0);
        let o = offset_origin(&self.p, &self.p_error, &self.n, dir);
        Ray::new(o, *dir)
    }

    pub fn spawn_ray_to(&self, p: &Point) -> Ray {
        let d = *p - self.p;
        assert!(d.x != 0.0 && d.y != 0.0 && d.z != 0.0);
        let o = offset_origin(&self.p, &self.p_error, &self.n, &d);
        Ray::segment(o, d, 1.0 - 1e-4)
    }
}

pub struct SurfaceInteraction<'a> {
    /// The point where the ray hit the primitive
    pub p: Point,
    /// Error bound for the intersection point
    pub p_error: Vector,
    /// Outgoing direction of the light at the intersection point (usually `-ray.d`)
    pub wo: Vector,
    /// Normal
    pub n: Vector,
    /// Texture coordinates
    pub uv: Point2f,
    /// Partial derivatives at the intersection point
    pub dpdu: Vector,
    pub dpdv: Vector,
    /// Partial derivaties of the normal
    pub dndu: Vector,
    pub dndv: Vector,
    /// Hit shape
    pub shape: &'a Shape,
    /// Hit primitive
    pub primitive: Option<&'a Primitive>,
    /// Shading information
    pub shading: Shading,
    /// BSDF of the surface at the intersection point
    pub bsdf: Option<Arc<BSDF>>,
}

impl<'a> SurfaceInteraction<'a> {
    pub fn new(p: Point,
               p_error: Vector,
               uv: Point2f,
               wo: Vector,
               dpdu: Vector,
               dpdv: Vector,
               shape: &Shape)
               -> SurfaceInteraction {
        let n = dpdu.cross(&dpdv).normalize();
        // TODO adjust normal for handedness
        SurfaceInteraction {
            p: p,
            p_error: p_error,
            n: n,
            uv: uv,
            wo: wo,
            dpdu: dpdu,
            dpdv: dpdv,
            dndu: na::zero(),
            dndv: na::zero(),
            shape: shape,
            primitive: None,
            // Initialize shading geometry from true geometry
            shading: Shading {
                n: n,
                dpdu: dpdu,
                dpdv: dpdv,
                dndu: na::zero(),
                dndv: na::zero(),
            },
            bsdf: None,
        }
    }

    pub fn le(&self, w: &Vector) -> Spectrum {
        self.primitive
            .and_then(|p| p.area_light())
            .map(|light| light.l(&self.into(), w))
            .unwrap_or_else(Spectrum::black)
    }

    pub fn transform(&self, t: &Transform) -> SurfaceInteraction<'a> {
        let (p, p_err) = transform::transform_point_with_error(t, &self.p, &self.p_error);
        SurfaceInteraction {
            p: p,
            p_error: p_err,
            n: transform::transform_normal(&self.n, t),
            uv: self.uv,
            wo: *t * self.wo,
            dpdu: *t * self.dpdu,
            dpdv: *t * self.dpdv,
            dndu: na::zero(),
            dndv: na::zero(),
            shape: self.shape,
            primitive: self.primitive,
            shading: Shading {
                n: transform::transform_normal(&self.n, t),
                dpdu: *t * self.dpdu,
                dpdv: *t * self.dpdv,
                dndu: na::zero(),
                dndv: na::zero(),
            },
            bsdf: self.bsdf.clone(),
        }
    }

    pub fn compute_scattering_functions(&mut self,
                                        _ray: &Ray,
                                        transport: TransportMode,
                                        allow_multiple_lobes: bool) {
        if let Some(primitive) = self.primitive {
            primitive.compute_scattering_functions(self, transport, allow_multiple_lobes);
        }
    }

    pub fn spawn_ray(&self, dir: &Vector) -> Ray {
        assert!(dir.x != 0.0 || dir.y != 0.0 || dir.z != 0.0);
        let o = offset_origin(&self.p, &self.p_error, &self.n, dir);
        Ray::new(o, *dir)
    }

    pub fn spawn_ray_to(&self, p: &Point) -> Ray {
        let d = *p - self.p;
        assert!(d.x != 0.0 || d.y != 0.0 || d.z != 0.0);
        let o = offset_origin(&self.p, &self.p_error, &self.n, &d);
        Ray::segment(o, d, 1.0 - 1e-4)
    }
}

fn offset_origin(p: &Point, p_err: &Vector, n: &Vector, w: &Vector) -> Point {
    let d = na::abs(n).dot(p_err);
    let mut offset = d * *n;
    if w.dot(n) < 0.0 {
        offset = -offset;
    }
    let mut po = *p + offset;
    for i in 0..3 {
        if offset[i] > 0.0 {
            po[i] = po[i].next();
        } else if offset[i] < 0.0 {
            po[i] = po[i].prev();
        }
    }
    po
}

impl<'a> From<SurfaceInteraction<'a>> for Interaction {
    fn from(si: SurfaceInteraction) -> Interaction {
        Interaction::new(si.p, si.p_error, si.wo, si.n)
    }
}

impl<'a> From<&'a SurfaceInteraction<'a>> for Interaction {
    fn from(si: &SurfaceInteraction) -> Interaction {
        Interaction::new(si.p, si.p_error, si.wo, si.n)
    }
}

/// Normal and partial derivatives used for shading. Can be different from geometric ones due to
/// bump mapping, etc.
pub struct Shading {
    pub n: Vector,
    pub dpdu: Vector,
    pub dpdv: Vector,
    pub dndu: Vector,
    pub dndv: Vector,
}

impl Default for Shading {
    fn default() -> Self {
        Shading {
            n: na::zero(),
            dpdu: na::zero(),
            dpdv: na::zero(),
            dndu: na::zero(),
            dndv: na::zero(),
        }
    }
}
