#[doc = "Register `raw_intr_stat` reader"]
pub type R = crate::R<RawIntrStatSpec>;
#[doc = "Register `raw_intr_stat` writer"]
pub type W = crate::W<RawIntrStatSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_intr_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_intr_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawIntrStatSpec;
impl crate::RegisterSpec for RawIntrStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_intr_stat::R`](R) reader structure"]
impl crate::Readable for RawIntrStatSpec {}
#[doc = "`write(|w| ..)` method takes [`raw_intr_stat::W`](W) writer structure"]
impl crate::Writable for RawIntrStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets raw_intr_stat to value 0"]
impl crate::Resettable for RawIntrStatSpec {
    const RESET_VALUE: u32 = 0;
}
