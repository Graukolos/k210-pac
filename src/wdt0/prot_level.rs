#[doc = "Register `prot_level` reader"]
pub type R = crate::R<ProtLevelSpec>;
#[doc = "Register `prot_level` writer"]
pub type W = crate::W<ProtLevelSpec>;
#[doc = "Field `prot_level` reader - prot_level"]
pub type ProtLevelR = crate::FieldReader;
#[doc = "Field `prot_level` writer - prot_level"]
pub type ProtLevelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - prot_level"]
    #[inline(always)]
    pub fn prot_level(&self) -> ProtLevelR {
        ProtLevelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - prot_level"]
    #[inline(always)]
    #[must_use]
    pub fn prot_level(&mut self) -> ProtLevelW<ProtLevelSpec> {
        ProtLevelW::new(self, 0)
    }
}
#[doc = "Protection level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`prot_level::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prot_level::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProtLevelSpec;
impl crate::RegisterSpec for ProtLevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prot_level::R`](R) reader structure"]
impl crate::Readable for ProtLevelSpec {}
#[doc = "`write(|w| ..)` method takes [`prot_level::W`](W) writer structure"]
impl crate::Writable for ProtLevelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets prot_level to value 0"]
impl crate::Resettable for ProtLevelSpec {
    const RESET_VALUE: u32 = 0;
}
