#[doc = "Register `de_en` reader"]
pub type R = crate::R<DeEnSpec>;
#[doc = "Register `de_en` writer"]
pub type W = crate::W<DeEnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DE Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`de_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`de_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeEnSpec;
impl crate::RegisterSpec for DeEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`de_en::R`](R) reader structure"]
impl crate::Readable for DeEnSpec {}
#[doc = "`write(|w| ..)` method takes [`de_en::W`](W) writer structure"]
impl crate::Writable for DeEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets de_en to value 0"]
impl crate::Resettable for DeEnSpec {
    const RESET_VALUE: u32 = 0;
}
