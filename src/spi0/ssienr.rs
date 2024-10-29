#[doc = "Register `ssienr` reader"]
pub type R = crate::R<SsienrSpec>;
#[doc = "Register `ssienr` writer"]
pub type W = crate::W<SsienrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssienr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssienr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsienrSpec;
impl crate::RegisterSpec for SsienrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssienr::R`](R) reader structure"]
impl crate::Readable for SsienrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssienr::W`](W) writer structure"]
impl crate::Writable for SsienrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ssienr to value 0"]
impl crate::Resettable for SsienrSpec {
    const RESET_VALUE: u32 = 0;
}
