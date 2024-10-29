#[doc = "Register `clr_rd_req` reader"]
pub type R = crate::R<ClrRdReqSpec>;
#[doc = "Field `clr` reader - CLR"]
pub type ClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RD_REQ Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_rd_req::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrRdReqSpec;
impl crate::RegisterSpec for ClrRdReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_rd_req::R`](R) reader structure"]
impl crate::Readable for ClrRdReqSpec {}
#[doc = "`reset()` method sets clr_rd_req to value 0"]
impl crate::Resettable for ClrRdReqSpec {
    const RESET_VALUE: u32 = 0;
}
