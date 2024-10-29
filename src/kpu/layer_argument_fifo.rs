#[doc = "Register `layer_argument_fifo` reader"]
pub type R = crate::R<LayerArgumentFifoSpec>;
#[doc = "Register `layer_argument_fifo` writer"]
pub type W = crate::W<LayerArgumentFifoSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Layer arguments FIFO: each layer is defined by writing 12 successive argument values to this register\n\nYou can [`read`](crate::Reg::read) this register and get [`layer_argument_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer_argument_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LayerArgumentFifoSpec;
impl crate::RegisterSpec for LayerArgumentFifoSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`layer_argument_fifo::R`](R) reader structure"]
impl crate::Readable for LayerArgumentFifoSpec {}
#[doc = "`write(|w| ..)` method takes [`layer_argument_fifo::W`](W) writer structure"]
impl crate::Writable for LayerArgumentFifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets layer_argument_fifo to value 0"]
impl crate::Resettable for LayerArgumentFifoSpec {
    const RESET_VALUE: u64 = 0;
}
