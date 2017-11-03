use std::sync::Arc;

use light_arena::Allocator;

use bsdf::{BxDF, Fresnel, LambertianReflection, MicrofacetReflection, SpecularReflection,
           SpecularTransmission, TrowbridgeReitzDistribution, BSDF};
use interaction::SurfaceInteraction;
use material::{Material, TransportMode};
use paramset::TextureParams;
use spectrum::Spectrum;
use texture::Texture;

type TextureSpectrum = Texture<Spectrum> + Send + Sync;
type TextureFloat = Texture<f32> + Send + Sync;

#[derive(Debug)]
pub struct UberMaterial {
    kd: Arc<TextureSpectrum>,
    ks: Arc<TextureSpectrum>,
    kr: Arc<TextureSpectrum>,
    kt: Arc<TextureSpectrum>,
    opacity: Arc<TextureSpectrum>,
    roughness: Arc<TextureFloat>,
    roughnessu: Option<Arc<TextureFloat>>,
    roughnessv: Option<Arc<TextureFloat>>,
    eta: Arc<TextureFloat>,
    // TODO bumpmap
    remap_roughness: bool,
}

impl UberMaterial {
    pub fn create(mp: &mut TextureParams) -> Arc<Material + Send + Sync> {
        let kd = mp.get_spectrum_texture("Kd", &Spectrum::from(0.25));
        let ks = mp.get_spectrum_texture("Kd", &Spectrum::from(0.25));
        let kr = mp.get_spectrum_texture("Kd", &Spectrum::from(0.0));
        let kt = mp.get_spectrum_texture("Kd", &Spectrum::from(0.0));
        let roughness = mp.get_float_texture("roughness", 0.1);
        let uroughness = mp.get_float_texture_or_none("uroughness");
        let vroughness = mp.get_float_texture_or_none("vroughness");
        let eta = mp.get_float_texture_or_none("eta")
            .unwrap_or_else(|| mp.get_float_texture("index", 1.5));
        let opacity = mp.get_spectrum_texture("opacity", &Spectrum::from(1.0));
        // TODO bumpmap
        let remap_roughness = mp.find_bool("remaproughness", true);

        Arc::new(UberMaterial {
                     kd,
                     ks,
                     kr,
                     kt,
                     opacity,
                     roughness,
                     roughnessu: uroughness,
                     roughnessv: vroughness,
                     eta,
                     remap_roughness,
                 })
    }
}

impl Material for UberMaterial {
    fn compute_scattering_functions<'a, 'b>(&self,
                                            si: &mut SurfaceInteraction<'a, 'b>,
                                            mode: TransportMode,
                                            _allow_multiple_lobes: bool,
                                            arena: &'b Allocator) {
        let mut bxdfs = arena.alloc_slice::<&BxDF>(8);
        let mut i = 0;

        let e = self.eta.evaluate(si);
        let op = self.opacity.evaluate(si).clamp();
        let t = (Spectrum::white() - op).clamp();

        let mut eta = e;
        if !t.is_black() {
            eta = 1.0;
            bxdfs[i] = arena <- SpecularTransmission::new(t, 1.0, 1.0, mode);
            i += 1;
        }

        let kd = op * self.kd.evaluate(si).clamp();
        if !kd.is_black() {
            bxdfs[i] = arena <- LambertianReflection::new(kd);
            i += 1;
        }

        let ks = op * self.ks.evaluate(si).clamp();
        if !ks.is_black() {
            let fresnel = arena <- Fresnel::dielectric(1.0, e);
            let mut roughu = self.roughnessu
                .as_ref()
                .unwrap_or(&self.roughness)
                .evaluate(si);
            let mut roughv = self.roughnessv
                .as_ref()
                .unwrap_or(&self.roughness)
                .evaluate(si);
            if self.remap_roughness {
                roughu = TrowbridgeReitzDistribution::roughness_to_alpha(roughu);
                roughv = TrowbridgeReitzDistribution::roughness_to_alpha(roughv);
            }
            let distrib = arena <- TrowbridgeReitzDistribution::new(roughu, roughv);
            bxdfs[i] = arena <- MicrofacetReflection::new(ks, distrib, fresnel);
            i += 1;
        }

        let kr = op * self.kr.evaluate(si).clamp();
        if !kr.is_black() {
            let fresnel = arena <- Fresnel::dielectric(1.0, e);
            bxdfs[i] = arena <-SpecularReflection::new(kr, fresnel);
            i += 1;
        }

        let kt = op * self.kt.evaluate(si).clamp();
        if !kt.is_black() {
            bxdfs[i] = arena <- SpecularTransmission::new(kt, 1.0, e, mode);
            i += 1;
        }

        unsafe {
            let ptr = bxdfs.as_mut_ptr();
            bxdfs = ::std::slice::from_raw_parts_mut(ptr, i);
        }

        let bsdf: BSDF<'b> = BSDF::new(si, eta, bxdfs);
        si.bsdf = Some(Arc::new(bsdf));
    }
}