#[doc = "Register `sar` reader"]
pub type R = crate::R<SarSpec>;
#[doc = "Register `sar` writer"]
pub type W = crate::W<SarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SAR Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SarSpec;
impl crate::RegisterSpec for SarSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`sar::R`](R) reader structure"]
impl crate::Readable for SarSpec {}
#[doc = "`write(|w| ..)` method takes [`sar::W`](W) writer structure"]
impl crate::Writable for SarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets sar to value 0"]
impl crate::Resettable for SarSpec {
    const RESET_VALUE: u64 = 0;
}
