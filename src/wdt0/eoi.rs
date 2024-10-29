#[doc = "Register `eoi` reader"]
pub type R = crate::R<EoiSpec>;
#[doc = "Register `eoi` writer"]
pub type W = crate::W<EoiSpec>;
#[doc = "Field `eoi` reader - eoi"]
pub type EoiR = crate::BitReader;
#[doc = "Field `eoi` writer - eoi"]
pub type EoiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - eoi"]
    #[inline(always)]
    pub fn eoi(&self) -> EoiR {
        EoiR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eoi"]
    #[inline(always)]
    #[must_use]
    pub fn eoi(&mut self) -> EoiW<EoiSpec> {
        EoiW::new(self, 0)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eoi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EoiSpec;
impl crate::RegisterSpec for EoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eoi::R`](R) reader structure"]
impl crate::Readable for EoiSpec {}
#[doc = "`write(|w| ..)` method takes [`eoi::W`](W) writer structure"]
impl crate::Writable for EoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets eoi to value 0"]
impl crate::Resettable for EoiSpec {
    const RESET_VALUE: u32 = 0;
}
