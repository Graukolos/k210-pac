#[doc = "Register `ss_scl_lcnt` reader"]
pub type R = crate::R<SsSclLcntSpec>;
#[doc = "Register `ss_scl_lcnt` writer"]
pub type W = crate::W<SsSclLcntSpec>;
#[doc = "Field `count` reader - COUNT"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `count` writer - COUNT"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - COUNT"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - COUNT"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<SsSclLcntSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Standard Speed Clock SCL Low Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_scl_lcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_scl_lcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsSclLcntSpec;
impl crate::RegisterSpec for SsSclLcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_scl_lcnt::R`](R) reader structure"]
impl crate::Readable for SsSclLcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_scl_lcnt::W`](W) writer structure"]
impl crate::Writable for SsSclLcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ss_scl_lcnt to value 0"]
impl crate::Resettable for SsSclLcntSpec {
    const RESET_VALUE: u32 = 0;
}
