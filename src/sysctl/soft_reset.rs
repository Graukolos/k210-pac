#[doc = "Register `soft_reset` reader"]
pub type R = crate::R<SoftResetSpec>;
#[doc = "Register `soft_reset` writer"]
pub type W = crate::W<SoftResetSpec>;
#[doc = "Field `soft_reset` reader - "]
pub type SoftResetR = crate::BitReader;
#[doc = "Field `soft_reset` writer - "]
pub type SoftResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn soft_reset(&self) -> SoftResetR {
        SoftResetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn soft_reset(&mut self) -> SoftResetW<SoftResetSpec> {
        SoftResetW::new(self, 0)
    }
}
#[doc = "Soft reset ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`soft_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftResetSpec;
impl crate::RegisterSpec for SoftResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soft_reset::R`](R) reader structure"]
impl crate::Readable for SoftResetSpec {}
#[doc = "`write(|w| ..)` method takes [`soft_reset::W`](W) writer structure"]
impl crate::Writable for SoftResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets soft_reset to value 0"]
impl crate::Resettable for SoftResetSpec {
    const RESET_VALUE: u32 = 0;
}
