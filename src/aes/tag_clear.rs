#[doc = "Register `tag_clear` reader"]
pub type R = crate::R<TagClearSpec>;
#[doc = "Register `tag_clear` writer"]
pub type W = crate::W<TagClearSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Tag clear (a write to this register clears the tag_chk status)\n\nYou can [`read`](crate::Reg::read) this register and get [`tag_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tag_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TagClearSpec;
impl crate::RegisterSpec for TagClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tag_clear::R`](R) reader structure"]
impl crate::Readable for TagClearSpec {}
#[doc = "`write(|w| ..)` method takes [`tag_clear::W`](W) writer structure"]
impl crate::Writable for TagClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tag_clear to value 0"]
impl crate::Resettable for TagClearSpec {
    const RESET_VALUE: u32 = 0;
}
