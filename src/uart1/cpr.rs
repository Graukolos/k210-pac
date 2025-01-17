#[doc = "Register `cpr` reader"]
pub type R = crate::R<CprSpec>;
#[doc = "Register `cpr` writer"]
pub type W = crate::W<CprSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Component Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CprSpec;
impl crate::RegisterSpec for CprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpr::R`](R) reader structure"]
impl crate::Readable for CprSpec {}
#[doc = "`write(|w| ..)` method takes [`cpr::W`](W) writer structure"]
impl crate::Writable for CprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cpr to value 0"]
impl crate::Resettable for CprSpec {
    const RESET_VALUE: u32 = 0;
}
