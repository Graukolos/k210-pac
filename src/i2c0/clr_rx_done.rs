#[doc = "Register `clr_rx_done` reader"]
pub type R = crate::R<ClrRxDoneSpec>;
#[doc = "Field `clr` reader - CLR"]
pub type ClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_DONE Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_rx_done::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrRxDoneSpec;
impl crate::RegisterSpec for ClrRxDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_rx_done::R`](R) reader structure"]
impl crate::Readable for ClrRxDoneSpec {}
#[doc = "`reset()` method sets clr_rx_done to value 0"]
impl crate::Resettable for ClrRxDoneSpec {
    const RESET_VALUE: u32 = 0;
}
