#[doc = "Register `sobuf_dma_rdata` reader"]
pub type R = crate::R<SobufDmaRdataSpec>;
#[doc = "Register `sobuf_dma_rdata` writer"]
pub type W = crate::W<SobufDmaRdataSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Read register for DMA to sample-out buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`sobuf_dma_rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sobuf_dma_rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SobufDmaRdataSpec;
impl crate::RegisterSpec for SobufDmaRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sobuf_dma_rdata::R`](R) reader structure"]
impl crate::Readable for SobufDmaRdataSpec {}
#[doc = "`write(|w| ..)` method takes [`sobuf_dma_rdata::W`](W) writer structure"]
impl crate::Writable for SobufDmaRdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sobuf_dma_rdata to value 0"]
impl crate::Resettable for SobufDmaRdataSpec {
    const RESET_VALUE: u32 = 0;
}
