#[doc = "Register `rxuicr` reader"]
pub type R = crate::R<RxuicrSpec>;
#[doc = "Register `rxuicr` writer"]
pub type W = crate::W<RxuicrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Receive FIFO Underflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxuicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxuicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxuicrSpec;
impl crate::RegisterSpec for RxuicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxuicr::R`](R) reader structure"]
impl crate::Readable for RxuicrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxuicr::W`](W) writer structure"]
impl crate::Writable for RxuicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rxuicr to value 0"]
impl crate::Resettable for RxuicrSpec {
    const RESET_VALUE: u32 = 0;
}
