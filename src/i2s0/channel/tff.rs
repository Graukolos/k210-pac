#[doc = "Register `tff` reader"]
pub type R = crate::R<TffSpec>;
#[doc = "Register `tff` writer"]
pub type W = crate::W<TffSpec>;
#[doc = "Transmit channel FIFO reset"]
pub use super::rff::Flush;
#[doc = "Field `rtxchfr` reader - Transmit channel FIFO reset"]
pub use super::rff::RxchfrR as RtxchfrR;
#[doc = "Field `rtxchfr` writer - Transmit channel FIFO reset"]
pub use super::rff::RxchfrW as RtxchfrW;
impl R {
    #[doc = "Bit 0 - Transmit channel FIFO reset"]
    #[inline(always)]
    pub fn rtxchfr(&self) -> RtxchfrR {
        RtxchfrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit channel FIFO reset"]
    #[inline(always)]
    #[must_use]
    pub fn rtxchfr(&mut self) -> RtxchfrW<TffSpec> {
        RtxchfrW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Flush Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TffSpec;
impl crate::RegisterSpec for TffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tff::R`](R) reader structure"]
impl crate::Readable for TffSpec {}
#[doc = "`write(|w| ..)` method takes [`tff::W`](W) writer structure"]
impl crate::Writable for TffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tff to value 0"]
impl crate::Resettable for TffSpec {
    const RESET_VALUE: u32 = 0;
}
