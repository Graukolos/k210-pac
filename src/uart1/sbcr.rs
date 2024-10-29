#[doc = "Register `sbcr` reader"]
pub type R = crate::R<SbcrSpec>;
#[doc = "Register `sbcr` writer"]
pub type W = crate::W<SbcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Shadow Break Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SbcrSpec;
impl crate::RegisterSpec for SbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbcr::R`](R) reader structure"]
impl crate::Readable for SbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sbcr::W`](W) writer structure"]
impl crate::Writable for SbcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sbcr to value 0"]
impl crate::Resettable for SbcrSpec {
    const RESET_VALUE: u32 = 0;
}
