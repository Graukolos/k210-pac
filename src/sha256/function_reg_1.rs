#[doc = "Register `function_reg_1` reader"]
pub type R = crate::R<FunctionReg1Spec>;
#[doc = "Register `function_reg_1` writer"]
pub type W = crate::W<FunctionReg1Spec>;
#[doc = "Field `dma_en` reader - SHA and DMA handshake signals enable. 1:enable; 0:disable"]
pub type DmaEnR = crate::BitReader;
#[doc = "Field `dma_en` writer - SHA and DMA handshake signals enable. 1:enable; 0:disable"]
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fifo_in_full` reader - 1:SHA256 input fifo is full; 0:not full"]
pub type FifoInFullR = crate::BitReader;
#[doc = "Field `fifo_in_full` writer - 1:SHA256 input fifo is full; 0:not full"]
pub type FifoInFullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SHA and DMA handshake signals enable. 1:enable; 0:disable"]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 1:SHA256 input fifo is full; 0:not full"]
    #[inline(always)]
    pub fn fifo_in_full(&self) -> FifoInFullR {
        FifoInFullR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SHA and DMA handshake signals enable. 1:enable; 0:disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_en(&mut self) -> DmaEnW<FunctionReg1Spec> {
        DmaEnW::new(self, 0)
    }
    #[doc = "Bit 8 - 1:SHA256 input fifo is full; 0:not full"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_in_full(&mut self) -> FifoInFullW<FunctionReg1Spec> {
        FifoInFullW::new(self, 8)
    }
}
#[doc = "Function configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`function_reg_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`function_reg_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FunctionReg1Spec;
impl crate::RegisterSpec for FunctionReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`function_reg_1::R`](R) reader structure"]
impl crate::Readable for FunctionReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`function_reg_1::W`](W) writer structure"]
impl crate::Writable for FunctionReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets function_reg_1 to value 0"]
impl crate::Resettable for FunctionReg1Spec {
    const RESET_VALUE: u32 = 0;
}
