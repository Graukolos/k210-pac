#[doc = "Register `ccvr` reader"]
pub type R = crate::R<CcvrSpec>;
#[doc = "Register `ccvr` writer"]
pub type W = crate::W<CcvrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Current Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcvrSpec;
impl crate::RegisterSpec for CcvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccvr::R`](R) reader structure"]
impl crate::Readable for CcvrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccvr::W`](W) writer structure"]
impl crate::Writable for CcvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ccvr to value 0"]
impl crate::Resettable for CcvrSpec {
    const RESET_VALUE: u32 = 0;
}
