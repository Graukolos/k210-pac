#[doc = "Register `intr_stat` reader"]
pub type R = crate::R<IntrStatSpec>;
#[doc = "Register `intr_stat` writer"]
pub type W = crate::W<IntrStatSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrStatSpec;
impl crate::RegisterSpec for IntrStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_stat::R`](R) reader structure"]
impl crate::Readable for IntrStatSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_stat::W`](W) writer structure"]
impl crate::Writable for IntrStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets intr_stat to value 0"]
impl crate::Resettable for IntrStatSpec {
    const RESET_VALUE: u32 = 0;
}
