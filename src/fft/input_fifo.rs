#[doc = "Register `input_fifo` reader"]
pub type R = crate::R<InputFifoSpec>;
#[doc = "Register `input_fifo` writer"]
pub type W = crate::W<InputFifoSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FFT input data fifo\n\nYou can [`read`](crate::Reg::read) this register and get [`input_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputFifoSpec;
impl crate::RegisterSpec for InputFifoSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`input_fifo::R`](R) reader structure"]
impl crate::Readable for InputFifoSpec {}
#[doc = "`write(|w| ..)` method takes [`input_fifo::W`](W) writer structure"]
impl crate::Writable for InputFifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets input_fifo to value 0"]
impl crate::Resettable for InputFifoSpec {
    const RESET_VALUE: u64 = 0;
}
