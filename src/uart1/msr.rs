#[doc = "Register `msr` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Register `msr` writer"]
pub type W = crate::W<MsrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Modem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`write(|w| ..)` method takes [`msr::W`](W) writer structure"]
impl crate::Writable for MsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets msr to value 0"]
impl crate::Resettable for MsrSpec {
    const RESET_VALUE: u32 = 0;
}
