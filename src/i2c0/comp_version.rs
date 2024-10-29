#[doc = "Register `comp_version` reader"]
pub type R = crate::R<CompVersionSpec>;
#[doc = "Field `value` reader - VALUE"]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "Component Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompVersionSpec;
impl crate::RegisterSpec for CompVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_version::R`](R) reader structure"]
impl crate::Readable for CompVersionSpec {}
#[doc = "`reset()` method sets comp_version to value 0"]
impl crate::Resettable for CompVersionSpec {
    const RESET_VALUE: u32 = 0;
}
