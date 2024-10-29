#[doc = "Register `rx_sample_delay` reader"]
pub type R = crate::R<RxSampleDelaySpec>;
#[doc = "Register `rx_sample_delay` writer"]
pub type W = crate::W<RxSampleDelaySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RX Sample Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_sample_delay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_sample_delay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxSampleDelaySpec;
impl crate::RegisterSpec for RxSampleDelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_sample_delay::R`](R) reader structure"]
impl crate::Readable for RxSampleDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`rx_sample_delay::W`](W) writer structure"]
impl crate::Writable for RxSampleDelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rx_sample_delay to value 0"]
impl crate::Resettable for RxSampleDelaySpec {
    const RESET_VALUE: u32 = 0;
}