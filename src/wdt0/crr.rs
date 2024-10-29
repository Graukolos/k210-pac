#[doc = "Register `crr` reader"]
pub type R = crate::R<CrrSpec>;
#[doc = "Register `crr` writer"]
pub type W = crate::W<CrrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Counter Restart Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrrSpec;
impl crate::RegisterSpec for CrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crr::R`](R) reader structure"]
impl crate::Readable for CrrSpec {}
#[doc = "`write(|w| ..)` method takes [`crr::W`](W) writer structure"]
impl crate::Writable for CrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets crr to value 0"]
impl crate::Resettable for CrrSpec {
    const RESET_VALUE: u32 = 0;
}
