#[doc = "Register `clr_start_det` reader"]
pub type R = crate::R<ClrStartDetSpec>;
#[doc = "Field `clr` reader - CLR"]
pub type ClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear START_DET Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_start_det::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrStartDetSpec;
impl crate::RegisterSpec for ClrStartDetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_start_det::R`](R) reader structure"]
impl crate::Readable for ClrStartDetSpec {}
#[doc = "`reset()` method sets clr_start_det to value 0"]
impl crate::Resettable for ClrStartDetSpec {
    const RESET_VALUE: u32 = 0;
}
