use std::sync::Arc;

use light_arena::Allocator;

use bsdf::{BxDF, Fresnel, FresnelSpecular, MicrofacetReflection, MicrofacetTransmission,
           SpecularReflection, SpecularTransmission, TrowbridgeReitzDistribution, BSDF};
use interaction::SurfaceInteraction;
use paramset::TextureParams;
use material::{Material, TransportMode};
use spectrum::Spectrum;
use texture::Texture;

pub struct GlassMaterial {
    kr: Arc<Texture<Spectrum> + Send + Sync>,
    kt: Arc<Texture<Spectrum> + Send + Sync>,
    u_roughness: Arc<Texture<f32> + Send + Sync>,
    v_roughness: Arc<Texture<f32> + Send + Sync>,
    index: Arc<Texture<f32> + Send + Sync>,
    remap_roughness: bool,
}

impl GlassMaterial {
    pub fn create(mp: &mut TextureParams) -> Arc<Material + Send + Sync> {
        info!("Creating Glass material");
        let Kr = mp.get_spectrum_texture("Kr", &Spectrum::white());
        let Kt = mp.get_spectrum_texture("Kt", &Spectrum::white());
        let eta = mp.get_float_texture_or_none("eta")
            .unwrap_or_else(|| mp.get_float_texture("index", 1.5));
        let rough_u = mp.get_float_texture("uroughness", 0.0);
        let rough_v = mp.get_float_texture("vroughness", 0.0);
        // TODO bumpmap
        let remap_roughness = mp.find_bool("remaproughness", true);

        Arc::new(GlassMaterial {
            kr: Kr,
            kt: Kt,
            u_roughness: rough_u,
            v_roughness: rough_v,
            index: eta,
            remap_roughness,
        })
    }
}


impl Material for GlassMaterial {
    fn compute_scattering_functions<'a, 'b>(
        &self,
        isect: &mut SurfaceInteraction<'a, 'b>,
        _mode: TransportMode,
        allow_multiple_lobes: bool,
        arena: &'b Allocator,
    ) {
        let eta = self.index.evaluate(isect);
        let mut u_rough = self.u_roughness.evaluate(isect);
        let mut v_rough = self.v_roughness.evaluate(isect);
        let r = self.kr.evaluate(isect);
        let t = self.kt.evaluate(isect);

        let mut bxdfs = arena.alloc_slice::<&BxDF>(8);
        let mut i = 0;

        if !r.is_black() || !t.is_black() {
            let is_specular = u_rough == 0.0 && v_rough == 0.0;
            if is_specular && allow_multiple_lobes {
                bxdfs[i] = arena <- FresnelSpecular::new();
                i += 1;
            } else {
                if self.remap_roughness {
                    u_rough = TrowbridgeReitzDistribution::roughness_to_alpha(u_rough);
                    v_rough = TrowbridgeReitzDistribution::roughness_to_alpha(v_rough);
                }
                if !r.is_black() {
                    let fresnel = arena <- Fresnel::dielectric(1.0, eta);
                    let bxdf: &'b BxDF = if is_specular {
                        arena <- SpecularReflection::new(r, fresnel)
                    } else {
                        let distrib = arena <- TrowbridgeReitzDistribution::new(u_rough, v_rough);
                        arena <- MicrofacetReflection::new(r, distrib, fresnel)
                    };
                    bxdfs[i] = bxdf;
                    i += 1;
                }
                if !t.is_black() {
                    let bxdf: &'b BxDF = if is_specular {
                        arena <- SpecularTransmission::new(t, 1.0, eta)
                    } else {
                        let distrib = arena <- TrowbridgeReitzDistribution::new(u_rough, v_rough);
                        arena <- MicrofacetTransmission::new(r, distrib, 1.0, eta)
                    };
                    bxdfs[i] = bxdf;
                    i += 1;
                }
            }
        }

        unsafe {
            let ptr = bxdfs.as_mut_ptr();
            bxdfs = ::std::slice::from_raw_parts_mut(ptr, i);
        }

        let bsdf = BSDF::new(isect, eta, bxdfs);
        isect.bsdf = Some(Arc::new(bsdf));
    }
}
