#[doc = "Register `output_fifo` reader"]
pub type R = crate::R<OutputFifoSpec>;
#[doc = "Register `output_fifo` writer"]
pub type W = crate::W<OutputFifoSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FFT output FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`output_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputFifoSpec;
impl crate::RegisterSpec for OutputFifoSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`output_fifo::R`](R) reader structure"]
impl crate::Readable for OutputFifoSpec {}
#[doc = "`write(|w| ..)` method takes [`output_fifo::W`](W) writer structure"]
impl crate::Writable for OutputFifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets output_fifo to value 0"]
impl crate::Resettable for OutputFifoSpec {
    const RESET_VALUE: u64 = 0;
}
