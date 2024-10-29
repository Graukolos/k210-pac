#[doc = "Register `xip_wrap_inst` reader"]
pub type R = crate::R<XipWrapInstSpec>;
#[doc = "Register `xip_wrap_inst` writer"]
pub type W = crate::W<XipWrapInstSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "XIP WRAP transfer opcode\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_wrap_inst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_wrap_inst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XipWrapInstSpec;
impl crate::RegisterSpec for XipWrapInstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_wrap_inst::R`](R) reader structure"]
impl crate::Readable for XipWrapInstSpec {}
#[doc = "`write(|w| ..)` method takes [`xip_wrap_inst::W`](W) writer structure"]
impl crate::Writable for XipWrapInstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets xip_wrap_inst to value 0"]
impl crate::Resettable for XipWrapInstSpec {
    const RESET_VALUE: u32 = 0;
}
