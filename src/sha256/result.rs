#[doc = "Register `result[%s]` reader"]
pub type R = crate::R<ResultSpec>;
#[doc = "Register `result[%s]` writer"]
pub type W = crate::W<ResultSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Calculated SHA256 return value\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultSpec;
impl crate::RegisterSpec for ResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for ResultSpec {}
#[doc = "`write(|w| ..)` method takes [`result::W`](W) writer structure"]
impl crate::Writable for ResultSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets result[%s]
to value 0"]
impl crate::Resettable for ResultSpec {
    const RESET_VALUE: u32 = 0;
}
