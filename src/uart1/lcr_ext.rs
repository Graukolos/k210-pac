#[doc = "Register `lcr_ext` reader"]
pub type R = crate::R<LcrExtSpec>;
#[doc = "Register `lcr_ext` writer"]
pub type W = crate::W<LcrExtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Line Control Register (Extended)\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr_ext::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr_ext::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrExtSpec;
impl crate::RegisterSpec for LcrExtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr_ext::R`](R) reader structure"]
impl crate::Readable for LcrExtSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr_ext::W`](W) writer structure"]
impl crate::Writable for LcrExtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lcr_ext to value 0"]
impl crate::Resettable for LcrExtSpec {
    const RESET_VALUE: u32 = 0;
}
