#[doc = "Register `fifo_ctrl` reader"]
pub type R = crate::R<FifoCtrlSpec>;
#[doc = "Register `fifo_ctrl` writer"]
pub type W = crate::W<FifoCtrlSpec>;
#[doc = "Field `dma_fifo_flush_n` reader - Flush DMA FIFO"]
pub type DmaFifoFlushNR = crate::BitReader;
#[doc = "Field `dma_fifo_flush_n` writer - Flush DMA FIFO"]
pub type DmaFifoFlushNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gs_fifo_flush_n` reader - Flush GS FIFO"]
pub type GsFifoFlushNR = crate::BitReader;
#[doc = "Field `gs_fifo_flush_n` writer - Flush GS FIFO"]
pub type GsFifoFlushNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_fifo_flush_n` reader - Flush configuration FIFO"]
pub type CfgFifoFlushNR = crate::BitReader;
#[doc = "Field `cfg_fifo_flush_n` writer - Flush configuration FIFO"]
pub type CfgFifoFlushNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmd_fifo_flush_n` reader - Flush command FIFO"]
pub type CmdFifoFlushNR = crate::BitReader;
#[doc = "Field `cmd_fifo_flush_n` writer - Flush command FIFO"]
pub type CmdFifoFlushNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `resp_fifo_flush_n` reader - Flush response FIFO"]
pub type RespFifoFlushNR = crate::BitReader;
#[doc = "Field `resp_fifo_flush_n` writer - Flush response FIFO"]
pub type RespFifoFlushNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flush DMA FIFO"]
    #[inline(always)]
    pub fn dma_fifo_flush_n(&self) -> DmaFifoFlushNR {
        DmaFifoFlushNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flush GS FIFO"]
    #[inline(always)]
    pub fn gs_fifo_flush_n(&self) -> GsFifoFlushNR {
        GsFifoFlushNR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flush configuration FIFO"]
    #[inline(always)]
    pub fn cfg_fifo_flush_n(&self) -> CfgFifoFlushNR {
        CfgFifoFlushNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flush command FIFO"]
    #[inline(always)]
    pub fn cmd_fifo_flush_n(&self) -> CmdFifoFlushNR {
        CmdFifoFlushNR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flush response FIFO"]
    #[inline(always)]
    pub fn resp_fifo_flush_n(&self) -> RespFifoFlushNR {
        RespFifoFlushNR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flush DMA FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn dma_fifo_flush_n(&mut self) -> DmaFifoFlushNW<FifoCtrlSpec> {
        DmaFifoFlushNW::new(self, 0)
    }
    #[doc = "Bit 1 - Flush GS FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn gs_fifo_flush_n(&mut self) -> GsFifoFlushNW<FifoCtrlSpec> {
        GsFifoFlushNW::new(self, 1)
    }
    #[doc = "Bit 2 - Flush configuration FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_fifo_flush_n(&mut self) -> CfgFifoFlushNW<FifoCtrlSpec> {
        CfgFifoFlushNW::new(self, 2)
    }
    #[doc = "Bit 3 - Flush command FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_fifo_flush_n(&mut self) -> CmdFifoFlushNW<FifoCtrlSpec> {
        CmdFifoFlushNW::new(self, 3)
    }
    #[doc = "Bit 4 - Flush response FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn resp_fifo_flush_n(&mut self) -> RespFifoFlushNW<FifoCtrlSpec> {
        RespFifoFlushNW::new(self, 4)
    }
}
#[doc = "FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoCtrlSpec;
impl crate::RegisterSpec for FifoCtrlSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`fifo_ctrl::R`](R) reader structure"]
impl crate::Readable for FifoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_ctrl::W`](W) writer structure"]
impl crate::Writable for FifoCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets fifo_ctrl to value 0"]
impl crate::Resettable for FifoCtrlSpec {
    const RESET_VALUE: u64 = 0;
}
