#[doc = "Register `left_rxtx` reader"]
pub type R = crate::R<LeftRxtxSpec>;
#[doc = "Register `left_rxtx` writer"]
pub type W = crate::W<LeftRxtxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Left Receive or Left Transmit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`left_rxtx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`left_rxtx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeftRxtxSpec;
impl crate::RegisterSpec for LeftRxtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`left_rxtx::R`](R) reader structure"]
impl crate::Readable for LeftRxtxSpec {}
#[doc = "`write(|w| ..)` method takes [`left_rxtx::W`](W) writer structure"]
impl crate::Writable for LeftRxtxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets left_rxtx to value 0"]
impl crate::Resettable for LeftRxtxSpec {
    const RESET_VALUE: u32 = 0;
}
