#[doc = "Register `status_raw` reader"]
pub type R = crate::R<StatusRawSpec>;
#[doc = "Register `status_raw` writer"]
pub type W = crate::W<StatusRawSpec>;
#[doc = "Field `fft_done` reader - FFT done"]
pub type FftDoneR = crate::BitReader;
#[doc = "Field `fft_done` writer - FFT done"]
pub type FftDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fft_work` reader - FFT work"]
pub type FftWorkR = crate::BitReader;
#[doc = "Field `fft_work` writer - FFT work"]
pub type FftWorkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FFT done"]
    #[inline(always)]
    pub fn fft_done(&self) -> FftDoneR {
        FftDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FFT work"]
    #[inline(always)]
    pub fn fft_work(&self) -> FftWorkR {
        FftWorkR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FFT done"]
    #[inline(always)]
    #[must_use]
    pub fn fft_done(&mut self) -> FftDoneW<StatusRawSpec> {
        FftDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - FFT work"]
    #[inline(always)]
    #[must_use]
    pub fn fft_work(&mut self) -> FftWorkW<StatusRawSpec> {
        FftWorkW::new(self, 1)
    }
}
#[doc = "FFT status raw\n\nYou can [`read`](crate::Reg::read) this register and get [`status_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusRawSpec;
impl crate::RegisterSpec for StatusRawSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`status_raw::R`](R) reader structure"]
impl crate::Readable for StatusRawSpec {}
#[doc = "`write(|w| ..)` method takes [`status_raw::W`](W) writer structure"]
impl crate::Writable for StatusRawSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets status_raw to value 0"]
impl crate::Resettable for StatusRawSpec {
    const RESET_VALUE: u64 = 0;
}
