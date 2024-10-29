#[doc = "Register `rxoicr` reader"]
pub type R = crate::R<RxoicrSpec>;
#[doc = "Register `rxoicr` writer"]
pub type W = crate::W<RxoicrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Receive FIFO Overflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxoicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxoicrSpec;
impl crate::RegisterSpec for RxoicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoicr::R`](R) reader structure"]
impl crate::Readable for RxoicrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxoicr::W`](W) writer structure"]
impl crate::Writable for RxoicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rxoicr to value 0"]
impl crate::Resettable for RxoicrSpec {
    const RESET_VALUE: u32 = 0;
}
