#[doc = "Register `det` reader"]
pub type R = crate::R<DetSpec>;
#[doc = "Register `det` writer"]
pub type W = crate::W<DetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DE Assertion Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`det::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`det::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DetSpec;
impl crate::RegisterSpec for DetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`det::R`](R) reader structure"]
impl crate::Readable for DetSpec {}
#[doc = "`write(|w| ..)` method takes [`det::W`](W) writer structure"]
impl crate::Writable for DetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets det to value 0"]
impl crate::Resettable for DetSpec {
    const RESET_VALUE: u32 = 0;
}
