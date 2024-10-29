#[doc = "Register `clr_intr` reader"]
pub type R = crate::R<ClrIntrSpec>;
#[doc = "Field `clr` reader - CLR"]
pub type ClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear Combined and Individual Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_intr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrIntrSpec;
impl crate::RegisterSpec for ClrIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_intr::R`](R) reader structure"]
impl crate::Readable for ClrIntrSpec {}
#[doc = "`reset()` method sets clr_intr to value 0"]
impl crate::Resettable for ClrIntrSpec {
    const RESET_VALUE: u32 = 0;
}
