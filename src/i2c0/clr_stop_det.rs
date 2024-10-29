#[doc = "Register `clr_stop_det` reader"]
pub type R = crate::R<ClrStopDetSpec>;
#[doc = "Field `clr` reader - CLR"]
pub type ClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear STOP_DET Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_stop_det::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrStopDetSpec;
impl crate::RegisterSpec for ClrStopDetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_stop_det::R`](R) reader structure"]
impl crate::Readable for ClrStopDetSpec {}
#[doc = "`reset()` method sets clr_stop_det to value 0"]
impl crate::Resettable for ClrStopDetSpec {
    const RESET_VALUE: u32 = 0;
}
