#[doc = "Register `comp_param_1` reader"]
pub type R = crate::R<CompParam1Spec>;
#[doc = "Register `comp_param_1` writer"]
pub type W = crate::W<CompParam1Spec>;
#[doc = "Field `always_en` reader - always_en"]
pub type AlwaysEnR = crate::BitReader;
#[doc = "Field `always_en` writer - always_en"]
pub type AlwaysEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dflt_rmod` reader - dflt_rmod"]
pub type DfltRmodR = crate::BitReader;
#[doc = "Field `dflt_rmod` writer - dflt_rmod"]
pub type DfltRmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dual_top` reader - dual_top"]
pub type DualTopR = crate::BitReader;
#[doc = "Field `dual_top` writer - dual_top"]
pub type DualTopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hc_rmod` reader - hc_rmod"]
pub type HcRmodR = crate::BitReader;
#[doc = "Field `hc_rmod` writer - hc_rmod"]
pub type HcRmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hc_rpl` reader - hc_rpl"]
pub type HcRplR = crate::BitReader;
#[doc = "Field `hc_rpl` writer - hc_rpl"]
pub type HcRplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hc_top` reader - hc_top"]
pub type HcTopR = crate::BitReader;
#[doc = "Field `hc_top` writer - hc_top"]
pub type HcTopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `use_fix_top` reader - use_fix_top"]
pub type UseFixTopR = crate::BitReader;
#[doc = "Field `use_fix_top` writer - use_fix_top"]
pub type UseFixTopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pause` reader - pause"]
pub type PauseR = crate::BitReader;
#[doc = "Field `pause` writer - pause"]
pub type PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `apb_data_width` reader - apb_data_width"]
pub type ApbDataWidthR = crate::FieldReader;
#[doc = "Field `apb_data_width` writer - apb_data_width"]
pub type ApbDataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dflt_rpl` reader - dflt_rpl"]
pub type DfltRplR = crate::FieldReader;
#[doc = "Field `dflt_rpl` writer - dflt_rpl"]
pub type DfltRplW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dflt_top` reader - dflt_top"]
pub type DfltTopR = crate::FieldReader;
#[doc = "Field `dflt_top` writer - dflt_top"]
pub type DfltTopW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dflt_top_init` reader - dflt_top_init"]
pub type DfltTopInitR = crate::FieldReader;
#[doc = "Field `dflt_top_init` writer - dflt_top_init"]
pub type DfltTopInitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cnt_width` reader - cnt_width"]
pub type CntWidthR = crate::FieldReader;
#[doc = "Field `cnt_width` writer - cnt_width"]
pub type CntWidthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - always_en"]
    #[inline(always)]
    pub fn always_en(&self) -> AlwaysEnR {
        AlwaysEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - dflt_rmod"]
    #[inline(always)]
    pub fn dflt_rmod(&self) -> DfltRmodR {
        DfltRmodR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - dual_top"]
    #[inline(always)]
    pub fn dual_top(&self) -> DualTopR {
        DualTopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hc_rmod"]
    #[inline(always)]
    pub fn hc_rmod(&self) -> HcRmodR {
        HcRmodR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hc_rpl"]
    #[inline(always)]
    pub fn hc_rpl(&self) -> HcRplR {
        HcRplR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - hc_top"]
    #[inline(always)]
    pub fn hc_top(&self) -> HcTopR {
        HcTopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - use_fix_top"]
    #[inline(always)]
    pub fn use_fix_top(&self) -> UseFixTopR {
        UseFixTopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pause"]
    #[inline(always)]
    pub fn pause(&self) -> PauseR {
        PauseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - apb_data_width"]
    #[inline(always)]
    pub fn apb_data_width(&self) -> ApbDataWidthR {
        ApbDataWidthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - dflt_rpl"]
    #[inline(always)]
    pub fn dflt_rpl(&self) -> DfltRplR {
        DfltRplR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 16:19 - dflt_top"]
    #[inline(always)]
    pub fn dflt_top(&self) -> DfltTopR {
        DfltTopR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - dflt_top_init"]
    #[inline(always)]
    pub fn dflt_top_init(&self) -> DfltTopInitR {
        DfltTopInitR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - cnt_width"]
    #[inline(always)]
    pub fn cnt_width(&self) -> CntWidthR {
        CntWidthR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - always_en"]
    #[inline(always)]
    #[must_use]
    pub fn always_en(&mut self) -> AlwaysEnW<CompParam1Spec> {
        AlwaysEnW::new(self, 0)
    }
    #[doc = "Bit 1 - dflt_rmod"]
    #[inline(always)]
    #[must_use]
    pub fn dflt_rmod(&mut self) -> DfltRmodW<CompParam1Spec> {
        DfltRmodW::new(self, 1)
    }
    #[doc = "Bit 2 - dual_top"]
    #[inline(always)]
    #[must_use]
    pub fn dual_top(&mut self) -> DualTopW<CompParam1Spec> {
        DualTopW::new(self, 2)
    }
    #[doc = "Bit 3 - hc_rmod"]
    #[inline(always)]
    #[must_use]
    pub fn hc_rmod(&mut self) -> HcRmodW<CompParam1Spec> {
        HcRmodW::new(self, 3)
    }
    #[doc = "Bit 4 - hc_rpl"]
    #[inline(always)]
    #[must_use]
    pub fn hc_rpl(&mut self) -> HcRplW<CompParam1Spec> {
        HcRplW::new(self, 4)
    }
    #[doc = "Bit 5 - hc_top"]
    #[inline(always)]
    #[must_use]
    pub fn hc_top(&mut self) -> HcTopW<CompParam1Spec> {
        HcTopW::new(self, 5)
    }
    #[doc = "Bit 6 - use_fix_top"]
    #[inline(always)]
    #[must_use]
    pub fn use_fix_top(&mut self) -> UseFixTopW<CompParam1Spec> {
        UseFixTopW::new(self, 6)
    }
    #[doc = "Bit 7 - pause"]
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PauseW<CompParam1Spec> {
        PauseW::new(self, 7)
    }
    #[doc = "Bits 8:9 - apb_data_width"]
    #[inline(always)]
    #[must_use]
    pub fn apb_data_width(&mut self) -> ApbDataWidthW<CompParam1Spec> {
        ApbDataWidthW::new(self, 8)
    }
    #[doc = "Bits 10:12 - dflt_rpl"]
    #[inline(always)]
    #[must_use]
    pub fn dflt_rpl(&mut self) -> DfltRplW<CompParam1Spec> {
        DfltRplW::new(self, 10)
    }
    #[doc = "Bits 16:19 - dflt_top"]
    #[inline(always)]
    #[must_use]
    pub fn dflt_top(&mut self) -> DfltTopW<CompParam1Spec> {
        DfltTopW::new(self, 16)
    }
    #[doc = "Bits 20:23 - dflt_top_init"]
    #[inline(always)]
    #[must_use]
    pub fn dflt_top_init(&mut self) -> DfltTopInitW<CompParam1Spec> {
        DfltTopInitW::new(self, 20)
    }
    #[doc = "Bits 24:28 - cnt_width"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_width(&mut self) -> CntWidthW<CompParam1Spec> {
        CntWidthW::new(self, 24)
    }
}
#[doc = "Component Parameters Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_param_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_param_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompParam1Spec;
impl crate::RegisterSpec for CompParam1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_param_1::R`](R) reader structure"]
impl crate::Readable for CompParam1Spec {}
#[doc = "`write(|w| ..)` method takes [`comp_param_1::W`](W) writer structure"]
impl crate::Writable for CompParam1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets comp_param_1 to value 0"]
impl crate::Resettable for CompParam1Spec {
    const RESET_VALUE: u32 = 0;
}
