#[doc = "Register `sat_counter` reader"]
pub type R = crate::R<SatCounterSpec>;
#[doc = "Register `sat_counter` writer"]
pub type W = crate::W<SatCounterSpec>;
#[doc = "Field `counter` reader - Counter"]
pub type CounterR = crate::FieldReader<u16>;
#[doc = "Field `counter` writer - Counter"]
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `total` reader - Total"]
pub type TotalR = crate::FieldReader<u16>;
#[doc = "Field `total` writer - Total"]
pub type TotalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter"]
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Total"]
    #[inline(always)]
    pub fn total(&self) -> TotalR {
        TotalR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> CounterW<SatCounterSpec> {
        CounterW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Total"]
    #[inline(always)]
    #[must_use]
    pub fn total(&mut self) -> TotalW<SatCounterSpec> {
        TotalW::new(self, 16)
    }
}
#[doc = "Saturation Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`sat_counter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sat_counter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SatCounterSpec;
impl crate::RegisterSpec for SatCounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sat_counter::R`](R) reader structure"]
impl crate::Readable for SatCounterSpec {}
#[doc = "`write(|w| ..)` method takes [`sat_counter::W`](W) writer structure"]
impl crate::Writable for SatCounterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sat_counter to value 0"]
impl crate::Resettable for SatCounterSpec {
    const RESET_VALUE: u32 = 0;
}
