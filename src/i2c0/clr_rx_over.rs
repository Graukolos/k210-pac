#[doc = "Register `clr_rx_over` reader"]
pub type R = crate::R<ClrRxOverSpec>;
#[doc = "Field `clr` reader - CLR"]
pub type ClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_OVER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_rx_over::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrRxOverSpec;
impl crate::RegisterSpec for ClrRxOverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_rx_over::R`](R) reader structure"]
impl crate::Readable for ClrRxOverSpec {}
#[doc = "`reset()` method sets clr_rx_over to value 0"]
impl crate::Resettable for ClrRxOverSpec {
    const RESET_VALUE: u32 = 0;
}
