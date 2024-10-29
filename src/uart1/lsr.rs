#[doc = "Register `lsr` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Register `lsr` writer"]
pub type W = crate::W<LsrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`write(|w| ..)` method takes [`lsr::W`](W) writer structure"]
impl crate::Writable for LsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lsr to value 0"]
impl crate::Resettable for LsrSpec {
    const RESET_VALUE: u32 = 0;
}
