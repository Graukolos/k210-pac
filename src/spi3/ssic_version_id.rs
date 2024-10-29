#[doc = "Register `ssic_version_id` reader"]
pub type R = crate::R<SsicVersionIdSpec>;
#[doc = "Register `ssic_version_id` writer"]
pub type W = crate::W<SsicVersionIdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DWC_ssi component version\n\nYou can [`read`](crate::Reg::read) this register and get [`ssic_version_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssic_version_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsicVersionIdSpec;
impl crate::RegisterSpec for SsicVersionIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssic_version_id::R`](R) reader structure"]
impl crate::Readable for SsicVersionIdSpec {}
#[doc = "`write(|w| ..)` method takes [`ssic_version_id::W`](W) writer structure"]
impl crate::Writable for SsicVersionIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ssic_version_id to value 0"]
impl crate::Resettable for SsicVersionIdSpec {
    const RESET_VALUE: u32 = 0;
}
