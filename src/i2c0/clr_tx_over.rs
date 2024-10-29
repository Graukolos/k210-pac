#[doc = "Register `clr_tx_over` reader"]
pub type R = crate::R<ClrTxOverSpec>;
#[doc = "Field `clr` reader - CLR"]
pub type ClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear TX_OVER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_tx_over::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrTxOverSpec;
impl crate::RegisterSpec for ClrTxOverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_tx_over::R`](R) reader structure"]
impl crate::Readable for ClrTxOverSpec {}
#[doc = "`reset()` method sets clr_tx_over to value 0"]
impl crate::Resettable for ClrTxOverSpec {
    const RESET_VALUE: u32 = 0;
}
