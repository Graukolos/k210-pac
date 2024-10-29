#[doc = "Register `xrxoicr` reader"]
pub type R = crate::R<XrxoicrSpec>;
#[doc = "Register `xrxoicr` writer"]
pub type W = crate::W<XrxoicrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "XIP Receive FIFO Overflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xrxoicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xrxoicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XrxoicrSpec;
impl crate::RegisterSpec for XrxoicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xrxoicr::R`](R) reader structure"]
impl crate::Readable for XrxoicrSpec {}
#[doc = "`write(|w| ..)` method takes [`xrxoicr::W`](W) writer structure"]
impl crate::Writable for XrxoicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets xrxoicr to value 0"]
impl crate::Resettable for XrxoicrSpec {
    const RESET_VALUE: u32 = 0;
}
