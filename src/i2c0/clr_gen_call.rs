#[doc = "Register `clr_gen_call` reader"]
pub type R = crate::R<ClrGenCallSpec>;
#[doc = "Field `clr` reader - CLR"]
pub type ClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "I2C Clear GEN_CALL Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_gen_call::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrGenCallSpec;
impl crate::RegisterSpec for ClrGenCallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_gen_call::R`](R) reader structure"]
impl crate::Readable for ClrGenCallSpec {}
#[doc = "`reset()` method sets clr_gen_call to value 0"]
impl crate::Resettable for ClrGenCallSpec {
    const RESET_VALUE: u32 = 0;
}
