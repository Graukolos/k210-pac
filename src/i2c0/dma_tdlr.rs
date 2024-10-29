#[doc = "Register `dma_tdlr` reader"]
pub type R = crate::R<DmaTdlrSpec>;
#[doc = "Register `dma_tdlr` writer"]
pub type W = crate::W<DmaTdlrSpec>;
#[doc = "Field `value` reader - VALUE"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `value` writer - VALUE"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - VALUE"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<DmaTdlrSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "DMA Transmit Data Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tdlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tdlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaTdlrSpec;
impl crate::RegisterSpec for DmaTdlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_tdlr::R`](R) reader structure"]
impl crate::Readable for DmaTdlrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_tdlr::W`](W) writer structure"]
impl crate::Writable for DmaTdlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dma_tdlr to value 0"]
impl crate::Resettable for DmaTdlrSpec {
    const RESET_VALUE: u32 = 0;
}
