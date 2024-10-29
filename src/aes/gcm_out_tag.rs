#[doc = "Register `gcm_out_tag[%s]` reader"]
pub type R = crate::R<GcmOutTagSpec>;
#[doc = "Register `gcm_out_tag[%s]` writer"]
pub type W = crate::W<GcmOutTagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Computed GCM output tag\n\nYou can [`read`](crate::Reg::read) this register and get [`gcm_out_tag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcm_out_tag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmOutTagSpec;
impl crate::RegisterSpec for GcmOutTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcm_out_tag::R`](R) reader structure"]
impl crate::Readable for GcmOutTagSpec {}
#[doc = "`write(|w| ..)` method takes [`gcm_out_tag::W`](W) writer structure"]
impl crate::Writable for GcmOutTagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gcm_out_tag[%s]
to value 0"]
impl crate::Resettable for GcmOutTagSpec {
    const RESET_VALUE: u32 = 0;
}
