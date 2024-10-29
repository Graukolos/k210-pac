#[doc = "Register `clr_tx_abrt` reader"]
pub type R = crate::R<ClrTxAbrtSpec>;
#[doc = "Field `clr` reader - CLR"]
pub type ClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear TX_ABRT Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_tx_abrt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrTxAbrtSpec;
impl crate::RegisterSpec for ClrTxAbrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_tx_abrt::R`](R) reader structure"]
impl crate::Readable for ClrTxAbrtSpec {}
#[doc = "`reset()` method sets clr_tx_abrt to value 0"]
impl crate::Resettable for ClrTxAbrtSpec {
    const RESET_VALUE: u32 = 0;
}
