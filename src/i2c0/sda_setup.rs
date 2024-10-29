#[doc = "Register `sda_setup` reader"]
pub type R = crate::R<SdaSetupSpec>;
#[doc = "Register `sda_setup` writer"]
pub type W = crate::W<SdaSetupSpec>;
#[doc = "Field `value` reader - VALUE"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `value` writer - VALUE"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VALUE"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<SdaSetupSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "SDA Setup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sda_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sda_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdaSetupSpec;
impl crate::RegisterSpec for SdaSetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda_setup::R`](R) reader structure"]
impl crate::Readable for SdaSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`sda_setup::W`](W) writer structure"]
impl crate::Writable for SdaSetupSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sda_setup to value 0"]
impl crate::Resettable for SdaSetupSpec {
    const RESET_VALUE: u32 = 0;
}
