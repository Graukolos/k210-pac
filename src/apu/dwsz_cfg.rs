#[doc = "Register `dwsz_cfg` reader"]
pub type R = crate::R<DwszCfgSpec>;
#[doc = "Register `dwsz_cfg` writer"]
pub type W = crate::W<DwszCfgSpec>;
#[doc = "Field `dir_dwn_siz_rate` reader - Down-sizing ratio used for direction searching"]
pub type DirDwnSizRateR = crate::FieldReader;
#[doc = "Field `dir_dwn_siz_rate` writer - Down-sizing ratio used for direction searching"]
pub type DirDwnSizRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `voc_dwn_siz_rate` reader - Down-sizing ratio used for voice stream generation"]
pub type VocDwnSizRateR = crate::FieldReader;
#[doc = "Field `voc_dwn_siz_rate` writer - Down-sizing ratio used for voice stream generation"]
pub type VocDwnSizRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `smpl_shift_bits` reader - Sample precision reduction when the source sound sample precision is 20/24/32 bits"]
pub type SmplShiftBitsR = crate::FieldReader;
#[doc = "Field `smpl_shift_bits` writer - Sample precision reduction when the source sound sample precision is 20/24/32 bits"]
pub type SmplShiftBitsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - Down-sizing ratio used for direction searching"]
    #[inline(always)]
    pub fn dir_dwn_siz_rate(&self) -> DirDwnSizRateR {
        DirDwnSizRateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Down-sizing ratio used for voice stream generation"]
    #[inline(always)]
    pub fn voc_dwn_siz_rate(&self) -> VocDwnSizRateR {
        VocDwnSizRateR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Sample precision reduction when the source sound sample precision is 20/24/32 bits"]
    #[inline(always)]
    pub fn smpl_shift_bits(&self) -> SmplShiftBitsR {
        SmplShiftBitsR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Down-sizing ratio used for direction searching"]
    #[inline(always)]
    #[must_use]
    pub fn dir_dwn_siz_rate(&mut self) -> DirDwnSizRateW<DwszCfgSpec> {
        DirDwnSizRateW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Down-sizing ratio used for voice stream generation"]
    #[inline(always)]
    #[must_use]
    pub fn voc_dwn_siz_rate(&mut self) -> VocDwnSizRateW<DwszCfgSpec> {
        VocDwnSizRateW::new(self, 4)
    }
    #[doc = "Bits 8:12 - Sample precision reduction when the source sound sample precision is 20/24/32 bits"]
    #[inline(always)]
    #[must_use]
    pub fn smpl_shift_bits(&mut self) -> SmplShiftBitsW<DwszCfgSpec> {
        SmplShiftBitsW::new(self, 8)
    }
}
#[doc = "Downsize Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dwsz_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwsz_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DwszCfgSpec;
impl crate::RegisterSpec for DwszCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwsz_cfg::R`](R) reader structure"]
impl crate::Readable for DwszCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dwsz_cfg::W`](W) writer structure"]
impl crate::Writable for DwszCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dwsz_cfg to value 0"]
impl crate::Resettable for DwszCfgSpec {
    const RESET_VALUE: u32 = 0;
}
