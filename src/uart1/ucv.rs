#[doc = "Register `ucv` reader"]
pub type R = crate::R<UcvSpec>;
#[doc = "Register `ucv` writer"]
pub type W = crate::W<UcvSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "UART Component Version\n\nYou can [`read`](crate::Reg::read) this register and get [`ucv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcvSpec;
impl crate::RegisterSpec for UcvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ucv::R`](R) reader structure"]
impl crate::Readable for UcvSpec {}
#[doc = "`write(|w| ..)` method takes [`ucv::W`](W) writer structure"]
impl crate::Writable for UcvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ucv to value 0"]
impl crate::Resettable for UcvSpec {
    const RESET_VALUE: u32 = 0;
}
