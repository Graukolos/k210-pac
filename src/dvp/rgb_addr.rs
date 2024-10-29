#[doc = "Register `rgb_addr` reader"]
pub type R = crate::R<RgbAddrSpec>;
#[doc = "Register `rgb_addr` writer"]
pub type W = crate::W<RgbAddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RGB_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`rgb_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgb_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgbAddrSpec;
impl crate::RegisterSpec for RgbAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgb_addr::R`](R) reader structure"]
impl crate::Readable for RgbAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`rgb_addr::W`](W) writer structure"]
impl crate::Writable for RgbAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rgb_addr to value 0"]
impl crate::Resettable for RgbAddrSpec {
    const RESET_VALUE: u32 = 0;
}
