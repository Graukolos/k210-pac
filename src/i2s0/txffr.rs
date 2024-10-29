#[doc = "Register `txffr` reader"]
pub type R = crate::R<TxffrSpec>;
#[doc = "Register `txffr` writer"]
pub type W = crate::W<TxffrSpec>;
#[doc = "Transmitter FIFO reset"]
pub use super::rxffr::Flush;
#[doc = "Field `rxffr` reader - Transmitter FIFO reset"]
pub use super::rxffr::RxffrR;
#[doc = "Field `rxffr` writer - Transmitter FIFO reset"]
pub use super::rxffr::RxffrW;
impl R {
    #[doc = "Bit 0 - Transmitter FIFO reset"]
    #[inline(always)]
    pub fn rxffr(&self) -> RxffrR {
        RxffrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter FIFO reset"]
    #[inline(always)]
    #[must_use]
    pub fn rxffr(&mut self) -> RxffrW<TxffrSpec> {
        RxffrW::new(self, 0)
    }
}
#[doc = "Transmitter Block FIFO Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txffr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txffr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxffrSpec;
impl crate::RegisterSpec for TxffrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txffr::R`](R) reader structure"]
impl crate::Readable for TxffrSpec {}
#[doc = "`write(|w| ..)` method takes [`txffr::W`](W) writer structure"]
impl crate::Writable for TxffrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets txffr to value 0"]
impl crate::Resettable for TxffrSpec {
    const RESET_VALUE: u32 = 0;
}
