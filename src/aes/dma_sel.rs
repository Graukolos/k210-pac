#[doc = "Register `dma_sel` reader"]
pub type R = crate::R<DmaSelSpec>;
#[doc = "Register `dma_sel` writer"]
pub type W = crate::W<DmaSelSpec>;
#[doc = "Field `dma_sel` reader - Output to DMA if set, to CPU otherwise"]
pub type DmaSelR = crate::BitReader;
#[doc = "Field `dma_sel` writer - Output to DMA if set, to CPU otherwise"]
pub type DmaSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Output to DMA if set, to CPU otherwise"]
    #[inline(always)]
    pub fn dma_sel(&self) -> DmaSelR {
        DmaSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output to DMA if set, to CPU otherwise"]
    #[inline(always)]
    #[must_use]
    pub fn dma_sel(&mut self) -> DmaSelW<DmaSelSpec> {
        DmaSelW::new(self, 0)
    }
}
#[doc = "DMA select\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSelSpec;
impl crate::RegisterSpec for DmaSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_sel::R`](R) reader structure"]
impl crate::Readable for DmaSelSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_sel::W`](W) writer structure"]
impl crate::Writable for DmaSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dma_sel to value 0"]
impl crate::Resettable for DmaSelSpec {
    const RESET_VALUE: u32 = 0;
}
