#[doc = "Register `dma_cr` reader"]
pub type R = crate::R<DmaCrSpec>;
#[doc = "Register `dma_cr` writer"]
pub type W = crate::W<DmaCrSpec>;
#[doc = "Field `RDMAE` reader - RDMAE"]
pub type RdmaeR = crate::BitReader;
#[doc = "Field `RDMAE` writer - RDMAE"]
pub type RdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDMAE` reader - TDMAE"]
pub type TdmaeR = crate::BitReader;
#[doc = "Field `TDMAE` writer - TDMAE"]
pub type TdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RDMAE"]
    #[inline(always)]
    pub fn rdmae(&self) -> RdmaeR {
        RdmaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TDMAE"]
    #[inline(always)]
    pub fn tdmae(&self) -> TdmaeR {
        TdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn rdmae(&mut self) -> RdmaeW<DmaCrSpec> {
        RdmaeW::new(self, 0)
    }
    #[doc = "Bit 1 - TDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn tdmae(&mut self) -> TdmaeW<DmaCrSpec> {
        TdmaeW::new(self, 1)
    }
}
#[doc = "I2C DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCrSpec;
impl crate::RegisterSpec for DmaCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cr::R`](R) reader structure"]
impl crate::Readable for DmaCrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_cr::W`](W) writer structure"]
impl crate::Writable for DmaCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dma_cr to value 0"]
impl crate::Resettable for DmaCrSpec {
    const RESET_VALUE: u32 = 0;
}
