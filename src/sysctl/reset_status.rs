#[doc = "Register `reset_status` reader"]
pub type R = crate::R<ResetStatusSpec>;
#[doc = "Register `reset_status` writer"]
pub type W = crate::W<ResetStatusSpec>;
#[doc = "Field `reset_sts_clr` reader - "]
pub type ResetStsClrR = crate::BitReader;
#[doc = "Field `reset_sts_clr` writer - "]
pub type ResetStsClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_reset_sts` reader - "]
pub type PinResetStsR = crate::BitReader;
#[doc = "Field `pin_reset_sts` writer - "]
pub type PinResetStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdt0_reset_sts` reader - "]
pub type Wdt0ResetStsR = crate::BitReader;
#[doc = "Field `wdt0_reset_sts` writer - "]
pub type Wdt0ResetStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdt1_reset_sts` reader - "]
pub type Wdt1ResetStsR = crate::BitReader;
#[doc = "Field `wdt1_reset_sts` writer - "]
pub type Wdt1ResetStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `soft_reset_sts` reader - "]
pub type SoftResetStsR = crate::BitReader;
#[doc = "Field `soft_reset_sts` writer - "]
pub type SoftResetStsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reset_sts_clr(&self) -> ResetStsClrR {
        ResetStsClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pin_reset_sts(&self) -> PinResetStsR {
        PinResetStsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wdt0_reset_sts(&self) -> Wdt0ResetStsR {
        Wdt0ResetStsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdt1_reset_sts(&self) -> Wdt1ResetStsR {
        Wdt1ResetStsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn soft_reset_sts(&self) -> SoftResetStsR {
        SoftResetStsR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reset_sts_clr(&mut self) -> ResetStsClrW<ResetStatusSpec> {
        ResetStsClrW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pin_reset_sts(&mut self) -> PinResetStsW<ResetStatusSpec> {
        PinResetStsW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn wdt0_reset_sts(&mut self) -> Wdt0ResetStsW<ResetStatusSpec> {
        Wdt0ResetStsW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wdt1_reset_sts(&mut self) -> Wdt1ResetStsW<ResetStatusSpec> {
        Wdt1ResetStsW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn soft_reset_sts(&mut self) -> SoftResetStsW<ResetStatusSpec> {
        SoftResetStsW::new(self, 4)
    }
}
#[doc = "Reset source status\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetStatusSpec;
impl crate::RegisterSpec for ResetStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_status::R`](R) reader structure"]
impl crate::Readable for ResetStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`reset_status::W`](W) writer structure"]
impl crate::Writable for ResetStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets reset_status to value 0"]
impl crate::Resettable for ResetStatusSpec {
    const RESET_VALUE: u32 = 0;
}
