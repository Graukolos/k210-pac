#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `status` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `fft_done` reader - FFT done"]
pub type FftDoneR = crate::BitReader;
#[doc = "Field `fft_done` writer - FFT done"]
pub type FftDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FFT done"]
    #[inline(always)]
    pub fn fft_done(&self) -> FftDoneR {
        FftDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FFT done"]
    #[inline(always)]
    #[must_use]
    pub fn fft_done(&mut self) -> FftDoneW<StatusSpec> {
        FftDoneW::new(self, 0)
    }
}
#[doc = "FFT status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u64 = 0;
}
