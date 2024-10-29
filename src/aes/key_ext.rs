#[doc = "Register `key_ext[%s]` reader"]
pub type R = crate::R<KeyExtSpec>;
#[doc = "Register `key_ext[%s]` writer"]
pub type W = crate::W<KeyExtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "5th-8th word of key\n\nYou can [`read`](crate::Reg::read) this register and get [`key_ext::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_ext::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyExtSpec;
impl crate::RegisterSpec for KeyExtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_ext::R`](R) reader structure"]
impl crate::Readable for KeyExtSpec {}
#[doc = "`write(|w| ..)` method takes [`key_ext::W`](W) writer structure"]
impl crate::Writable for KeyExtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets key_ext[%s]
to value 0"]
impl crate::Resettable for KeyExtSpec {
    const RESET_VALUE: u32 = 0;
}
