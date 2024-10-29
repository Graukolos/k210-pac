#[doc = "Register `dummy` reader"]
pub type R = crate::R<DummySpec>;
#[doc = "Register `dummy` writer"]
pub type W = crate::W<DummySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Dummy register: this peripheral is not implemented yet\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DummySpec;
impl crate::RegisterSpec for DummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dummy::R`](R) reader structure"]
impl crate::Readable for DummySpec {}
#[doc = "`write(|w| ..)` method takes [`dummy::W`](W) writer structure"]
impl crate::Writable for DummySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dummy to value 0"]
impl crate::Resettable for DummySpec {
    const RESET_VALUE: u32 = 0;
}
