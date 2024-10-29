#[doc = "Register `gcm_in_tag[%s]` reader"]
pub type R = crate::R<GcmInTagSpec>;
#[doc = "Register `gcm_in_tag[%s]` writer"]
pub type W = crate::W<GcmInTagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GCM input tag for comparison with the calculated tag\n\nYou can [`read`](crate::Reg::read) this register and get [`gcm_in_tag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcm_in_tag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmInTagSpec;
impl crate::RegisterSpec for GcmInTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcm_in_tag::R`](R) reader structure"]
impl crate::Readable for GcmInTagSpec {}
#[doc = "`write(|w| ..)` method takes [`gcm_in_tag::W`](W) writer structure"]
impl crate::Writable for GcmInTagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gcm_in_tag[%s]
to value 0"]
impl crate::Resettable for GcmInTagSpec {
    const RESET_VALUE: u32 = 0;
}
