#[doc = "Register `xip_mode_bits` reader"]
pub type R = crate::R<XipModeBitsSpec>;
#[doc = "Register `xip_mode_bits` writer"]
pub type W = crate::W<XipModeBitsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "XIP Mode bits\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_mode_bits::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_mode_bits::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XipModeBitsSpec;
impl crate::RegisterSpec for XipModeBitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_mode_bits::R`](R) reader structure"]
impl crate::Readable for XipModeBitsSpec {}
#[doc = "`write(|w| ..)` method takes [`xip_mode_bits::W`](W) writer structure"]
impl crate::Writable for XipModeBitsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets xip_mode_bits to value 0"]
impl crate::Resettable for XipModeBitsSpec {
    const RESET_VALUE: u32 = 0;
}
