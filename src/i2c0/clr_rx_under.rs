#[doc = "Register `clr_rx_under` reader"]
pub type R = crate::R<ClrRxUnderSpec>;
#[doc = "Field `clr` reader - CLR"]
pub type ClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_UNDER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_rx_under::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrRxUnderSpec;
impl crate::RegisterSpec for ClrRxUnderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_rx_under::R`](R) reader structure"]
impl crate::Readable for ClrRxUnderSpec {}
#[doc = "`reset()` method sets clr_rx_under to value 0"]
impl crate::Resettable for ClrRxUnderSpec {
    const RESET_VALUE: u32 = 0;
}
