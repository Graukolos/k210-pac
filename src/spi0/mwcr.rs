#[doc = "Register `mwcr` reader"]
pub type R = crate::R<MwcrSpec>;
#[doc = "Register `mwcr` writer"]
pub type W = crate::W<MwcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Microwire Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mwcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwcrSpec;
impl crate::RegisterSpec for MwcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mwcr::R`](R) reader structure"]
impl crate::Readable for MwcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mwcr::W`](W) writer structure"]
impl crate::Writable for MwcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mwcr to value 0"]
impl crate::Resettable for MwcrSpec {
    const RESET_VALUE: u32 = 0;
}
