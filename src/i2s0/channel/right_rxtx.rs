#[doc = "Register `right_rxtx` reader"]
pub type R = crate::R<RightRxtxSpec>;
#[doc = "Register `right_rxtx` writer"]
pub type W = crate::W<RightRxtxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Right Receive or Right Transmit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`right_rxtx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`right_rxtx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RightRxtxSpec;
impl crate::RegisterSpec for RightRxtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`right_rxtx::R`](R) reader structure"]
impl crate::Readable for RightRxtxSpec {}
#[doc = "`write(|w| ..)` method takes [`right_rxtx::W`](W) writer structure"]
impl crate::Writable for RightRxtxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets right_rxtx to value 0"]
impl crate::Resettable for RightRxtxSpec {
    const RESET_VALUE: u32 = 0;
}
