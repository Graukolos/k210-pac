#[doc = "Register `tfcr` reader"]
pub type R = crate::R<TfcrSpec>;
#[doc = "Register `tfcr` writer"]
pub type W = crate::W<TfcrSpec>;
#[doc = "Trigger level in the TX FIFO at which the transmitter data available interrupt generate"]
pub use super::rfcr::Level;
#[doc = "Field `txchet` reader - Trigger level in the TX FIFO at which the transmitter data available interrupt generate"]
pub use super::rfcr::RxchdtR as TxchetR;
#[doc = "Field `txchet` writer - Trigger level in the TX FIFO at which the transmitter data available interrupt generate"]
pub use super::rfcr::RxchdtW as TxchetW;
impl R {
    #[doc = "Bits 0:3 - Trigger level in the TX FIFO at which the transmitter data available interrupt generate"]
    #[inline(always)]
    pub fn txchet(&self) -> TxchetR {
        TxchetR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trigger level in the TX FIFO at which the transmitter data available interrupt generate"]
    #[inline(always)]
    #[must_use]
    pub fn txchet(&mut self) -> TxchetW<TfcrSpec> {
        TxchetW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfcrSpec;
impl crate::RegisterSpec for TfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfcr::R`](R) reader structure"]
impl crate::Readable for TfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tfcr::W`](W) writer structure"]
impl crate::Writable for TfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tfcr to value 0"]
impl crate::Resettable for TfcrSpec {
    const RESET_VALUE: u32 = 0;
}
