#[doc = "Register `xip_ser` reader"]
pub type R = crate::R<XipSerSpec>;
#[doc = "Register `xip_ser` writer"]
pub type W = crate::W<XipSerSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "XIP Slave Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_ser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_ser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XipSerSpec;
impl crate::RegisterSpec for XipSerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_ser::R`](R) reader structure"]
impl crate::Readable for XipSerSpec {}
#[doc = "`write(|w| ..)` method takes [`xip_ser::W`](W) writer structure"]
impl crate::Writable for XipSerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets xip_ser to value 0"]
impl crate::Resettable for XipSerSpec {
    const RESET_VALUE: u32 = 0;
}
