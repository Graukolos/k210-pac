#[doc = "Register `current_count` reader"]
pub type R = crate::R<CurrentCountSpec>;
#[doc = "Register `current_count` writer"]
pub type W = crate::W<CurrentCountSpec>;
#[doc = "Field `count` reader - RTC counter current value"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `count` writer - RTC counter current value"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RTC counter current value"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC counter current value"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<CurrentCountSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Timer counter current value\n\nYou can [`read`](crate::Reg::read) this register and get [`current_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`current_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrentCountSpec;
impl crate::RegisterSpec for CurrentCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current_count::R`](R) reader structure"]
impl crate::Readable for CurrentCountSpec {}
#[doc = "`write(|w| ..)` method takes [`current_count::W`](W) writer structure"]
impl crate::Writable for CurrentCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets current_count to value 0"]
impl crate::Resettable for CurrentCountSpec {
    const RESET_VALUE: u32 = 0;
}
