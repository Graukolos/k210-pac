#[doc = "Register `fft_cfg` reader"]
pub type R = crate::R<FftCfgSpec>;
#[doc = "Register `fft_cfg` writer"]
pub type W = crate::W<FftCfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FFT Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fft_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fft_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FftCfgSpec;
impl crate::RegisterSpec for FftCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fft_cfg::R`](R) reader structure"]
impl crate::Readable for FftCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fft_cfg::W`](W) writer structure"]
impl crate::Writable for FftCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fft_cfg to value 0"]
impl crate::Resettable for FftCfgSpec {
    const RESET_VALUE: u32 = 0;
}
