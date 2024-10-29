#[doc = "Register `dma_sel1` reader"]
pub type R = crate::R<DmaSel1Spec>;
#[doc = "Register `dma_sel1` writer"]
pub type W = crate::W<DmaSel1Spec>;
#[doc = "Field `dma_sel5` reader - "]
pub use super::dma_sel0::DmaSel0R as DmaSel5R;
#[doc = "Field `dma_sel5` writer - "]
pub use super::dma_sel0::DmaSel0W as DmaSel5W;
#[doc = ""]
pub use super::dma_sel0::Dmaselect;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn dma_sel5(&self) -> DmaSel5R {
        DmaSel5R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn dma_sel5(&mut self) -> DmaSel5W<DmaSel1Spec> {
        DmaSel5W::new(self, 0)
    }
}
#[doc = "DMA handshake selector\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSel1Spec;
impl crate::RegisterSpec for DmaSel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_sel1::R`](R) reader structure"]
impl crate::Readable for DmaSel1Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_sel1::W`](W) writer structure"]
impl crate::Writable for DmaSel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dma_sel1 to value 0"]
impl crate::Resettable for DmaSel1Spec {
    const RESET_VALUE: u32 = 0;
}
