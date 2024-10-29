#[doc = "Register `tag_in_flag` reader"]
pub type R = crate::R<TagInFlagSpec>;
#[doc = "Register `tag_in_flag` writer"]
pub type W = crate::W<TagInFlagSpec>;
#[doc = "GCM tag can be written to gcm_in_tag when this flag is set"]
pub use super::data_in_flag::CanInput;
#[doc = "Field `tag_in_flag` reader - GCM tag can be written to gcm_in_tag when this flag is set"]
pub use super::data_in_flag::DataInFlagR as TagInFlagR;
#[doc = "Field `tag_in_flag` writer - GCM tag can be written to gcm_in_tag when this flag is set"]
pub use super::data_in_flag::DataInFlagW as TagInFlagW;
impl R {
    #[doc = "Bit 0 - GCM tag can be written to gcm_in_tag when this flag is set"]
    #[inline(always)]
    pub fn tag_in_flag(&self) -> TagInFlagR {
        TagInFlagR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GCM tag can be written to gcm_in_tag when this flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn tag_in_flag(&mut self) -> TagInFlagW<TagInFlagSpec> {
        TagInFlagW::new(self, 0)
    }
}
#[doc = "Can input tag (when using GCM)\n\nYou can [`read`](crate::Reg::read) this register and get [`tag_in_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tag_in_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TagInFlagSpec;
impl crate::RegisterSpec for TagInFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tag_in_flag::R`](R) reader structure"]
impl crate::Readable for TagInFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`tag_in_flag::W`](W) writer structure"]
impl crate::Writable for TagInFlagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tag_in_flag to value 0"]
impl crate::Resettable for TagInFlagSpec {
    const RESET_VALUE: u32 = 0;
}
