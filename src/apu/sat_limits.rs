#[doc = "Register `sat_limits` reader"]
pub type R = crate::R<SatLimitsSpec>;
#[doc = "Register `sat_limits` writer"]
pub type W = crate::W<SatLimitsSpec>;
#[doc = "Field `upper` reader - Upper limit"]
pub type UpperR = crate::FieldReader<u16>;
#[doc = "Field `upper` writer - Upper limit"]
pub type UpperW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bottom` reader - Bottom limit"]
pub type BottomR = crate::FieldReader<u16>;
#[doc = "Field `bottom` writer - Bottom limit"]
pub type BottomW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Upper limit"]
    #[inline(always)]
    pub fn upper(&self) -> UpperR {
        UpperR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Bottom limit"]
    #[inline(always)]
    pub fn bottom(&self) -> BottomR {
        BottomR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Upper limit"]
    #[inline(always)]
    #[must_use]
    pub fn upper(&mut self) -> UpperW<SatLimitsSpec> {
        UpperW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Bottom limit"]
    #[inline(always)]
    #[must_use]
    pub fn bottom(&mut self) -> BottomW<SatLimitsSpec> {
        BottomW::new(self, 16)
    }
}
#[doc = "Saturation Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`sat_limits::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sat_limits::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SatLimitsSpec;
impl crate::RegisterSpec for SatLimitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sat_limits::R`](R) reader structure"]
impl crate::Readable for SatLimitsSpec {}
#[doc = "`write(|w| ..)` method takes [`sat_limits::W`](W) writer structure"]
impl crate::Writable for SatLimitsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sat_limits to value 0"]
impl crate::Resettable for SatLimitsSpec {
    const RESET_VALUE: u32 = 0;
}
