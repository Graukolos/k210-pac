#[doc = "Register `fifo_ctrl` reader"]
pub type R = crate::R<FifoCtrlSpec>;
#[doc = "Register `fifo_ctrl` writer"]
pub type W = crate::W<FifoCtrlSpec>;
#[doc = "Field `resp_fifo_flush` reader - Response memory initialization flag"]
pub type RespFifoFlushR = crate::BitReader;
#[doc = "Field `resp_fifo_flush` writer - Response memory initialization flag"]
pub type RespFifoFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmd_fifo_flush` reader - Command memory initialization flag"]
pub type CmdFifoFlushR = crate::BitReader;
#[doc = "Field `cmd_fifo_flush` writer - Command memory initialization flag"]
pub type CmdFifoFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gs_fifo_flush` reader - Output interface memory initialization flag"]
pub type GsFifoFlushR = crate::BitReader;
#[doc = "Field `gs_fifo_flush` writer - Output interface memory initialization flag"]
pub type GsFifoFlushW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Response memory initialization flag"]
    #[inline(always)]
    pub fn resp_fifo_flush(&self) -> RespFifoFlushR {
        RespFifoFlushR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command memory initialization flag"]
    #[inline(always)]
    pub fn cmd_fifo_flush(&self) -> CmdFifoFlushR {
        CmdFifoFlushR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output interface memory initialization flag"]
    #[inline(always)]
    pub fn gs_fifo_flush(&self) -> GsFifoFlushR {
        GsFifoFlushR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Response memory initialization flag"]
    #[inline(always)]
    #[must_use]
    pub fn resp_fifo_flush(&mut self) -> RespFifoFlushW<FifoCtrlSpec> {
        RespFifoFlushW::new(self, 0)
    }
    #[doc = "Bit 1 - Command memory initialization flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_fifo_flush(&mut self) -> CmdFifoFlushW<FifoCtrlSpec> {
        CmdFifoFlushW::new(self, 1)
    }
    #[doc = "Bit 2 - Output interface memory initialization flag"]
    #[inline(always)]
    #[must_use]
    pub fn gs_fifo_flush(&mut self) -> GsFifoFlushW<FifoCtrlSpec> {
        GsFifoFlushW::new(self, 2)
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
