use std::fmt::Debug;
use std::ops::Mul;
use std::sync::Arc;

use crate::interaction::SurfaceInteraction;
use crate::paramset::TextureParams;
use crate::spectrum::Spectrum;
use crate::texture::Texture;

#[derive(Debug)]
pub struct ScaleTexture<T> {
    tex1: Arc<dyn Texture<T>>,
    tex2: Arc<dyn Texture<T>>,
}

impl<T> Texture<T> for ScaleTexture<T>
where
    T: Debug,
    T: Send,
    T: Sync,
    T: Mul<Output = T>,
{
    fn evaluate(&self, si: &SurfaceInteraction<'_, '_>) -> T {
        self.tex1.evaluate(si) * self.tex2.evaluate(si)
    }
}

impl ScaleTexture<Spectrum> {
    pub fn create(tp: &TextureParams<'_>) -> ScaleTexture<Spectrum> {
        let tex1 = tp.get_spectrum_texture("tex1", &Spectrum::white());
        let tex2 = tp.get_spectrum_texture("tex2", &Spectrum::white());

        ScaleTexture { tex1, tex2 }
    }
}

impl ScaleTexture<f32> {
    pub fn create(tp: &TextureParams<'_>) -> ScaleTexture<f32> {
        let tex1 = tp.get_float_texture("tex1", 1.0);
        let tex2 = tp.get_float_texture("tex2", 1.0);

        ScaleTexture { tex1, tex2 }
    }
}
