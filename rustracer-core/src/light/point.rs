use std::f32::consts::PI;
use std::sync::Arc;

use num::Zero;

use crate::interaction::Interaction;
use crate::light::{Light, LightFlags, VisibilityTester};
use crate::paramset::ParamSet;
use crate::spectrum::Spectrum;
use crate::{Point2f, Point3f, Transform, Vector3f};

#[derive(Debug)]
pub struct PointLight {
    id: u32,
    pos: Point3f,
    emission_colour: Spectrum,
}

impl PointLight {
    pub fn new(p: Point3f, ec: Spectrum) -> PointLight {
        PointLight {
            id: super::get_next_id(),
            pos: p,
            emission_colour: ec,
        }
    }

    pub fn create(l2w: &Transform, params: &ParamSet) -> Arc<dyn Light> {
        let I = params.find_one_spectrum("I", Spectrum::white());
        let scale = params.find_one_spectrum("scale", Spectrum::white());
        let p = params.find_one_point3f("from", Point3f::zero());

        let t = &Transform::translate(&Vector3f::new(p.x, p.y, p.z)) * l2w;
        Arc::new(PointLight::new(&t * &Point3f::zero(), I * scale))
    }
}

impl Light for PointLight {
    fn id(&self) -> u32 {
        self.id
    }

    fn sample_li(
        &self,
        isect: &Interaction,
        _u: Point2f,
    ) -> (Spectrum, Vector3f, f32, VisibilityTester) {
        let wi = self.pos - isect.p;
        let r2 = wi.length_squared();
        let l_i = self.emission_colour / (4.0 * PI * r2);
        let vt = VisibilityTester::new(*isect, Interaction::from_point(&self.pos));

        (l_i, wi.normalize(), 1.0, vt)
    }

    fn pdf_li(&self, _si: &Interaction, _wi: &Vector3f) -> f32 {
        0.0
    }

    fn n_samples(&self) -> u32 {
        1
    }

    fn flags(&self) -> LightFlags {
        LightFlags::DELTA_POSITION
    }

    fn power(&self) -> Spectrum {
        4.0 * PI * self.emission_colour
    }
}
