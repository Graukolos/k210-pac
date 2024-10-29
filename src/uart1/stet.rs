#[doc = "Register `stet` reader"]
pub type R = crate::R<StetSpec>;
#[doc = "Register `stet` writer"]
pub type W = crate::W<StetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Shadow TX Empty Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stet::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stet::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StetSpec;
impl crate::RegisterSpec for StetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stet::R`](R) reader structure"]
impl crate::Readable for StetSpec {}
#[doc = "`write(|w| ..)` method takes [`stet::W`](W) writer structure"]
impl crate::Writable for StetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stet to value 0"]
impl crate::Resettable for StetSpec {
    const RESET_VALUE: u32 = 0;
}
