#[doc = "Register `lcr` reader"]
pub type R = crate::R<LcrSpec>;
#[doc = "Register `lcr` writer"]
pub type W = crate::W<LcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrSpec;
impl crate::RegisterSpec for LcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lcr to value 0"]
impl crate::Resettable for LcrSpec {
    const RESET_VALUE: u32 = 0;
}
