#[doc = "Register `clr_activity` reader"]
pub type R = crate::R<ClrActivitySpec>;
#[doc = "Field `clr` reader - CLR"]
pub type ClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear ACTIVITY Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_activity::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrActivitySpec;
impl crate::RegisterSpec for ClrActivitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_activity::R`](R) reader structure"]
impl crate::Readable for ClrActivitySpec {}
#[doc = "`reset()` method sets clr_activity to value 0"]
impl crate::Resettable for ClrActivitySpec {
    const RESET_VALUE: u32 = 0;
}
