#[doc = "Register `txoicr` reader"]
pub type R = crate::R<TxoicrSpec>;
#[doc = "Register `txoicr` writer"]
pub type W = crate::W<TxoicrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Transmit FIFO Overflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txoicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txoicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxoicrSpec;
impl crate::RegisterSpec for TxoicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txoicr::R`](R) reader structure"]
impl crate::Readable for TxoicrSpec {}
#[doc = "`write(|w| ..)` method takes [`txoicr::W`](W) writer structure"]
impl crate::Writable for TxoicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets txoicr to value 0"]
impl crate::Resettable for TxoicrSpec {
    const RESET_VALUE: u32 = 0;
}
