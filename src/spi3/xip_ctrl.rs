#[doc = "Register `xip_ctrl` reader"]
pub type R = crate::R<XipCtrlSpec>;
#[doc = "Register `xip_ctrl` writer"]
pub type W = crate::W<XipCtrlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "XIP Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XipCtrlSpec;
impl crate::RegisterSpec for XipCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_ctrl::R`](R) reader structure"]
impl crate::Readable for XipCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`xip_ctrl::W`](W) writer structure"]
impl crate::Writable for XipCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets xip_ctrl to value 0"]
impl crate::Resettable for XipCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
