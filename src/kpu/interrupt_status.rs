#[doc = "Register `interrupt_status` reader"]
pub type R = crate::R<InterruptStatusSpec>;
#[doc = "Register `interrupt_status` writer"]
pub type W = crate::W<InterruptStatusSpec>;
#[doc = "Field `calc_done` reader - Interrupt raised when calculation is done"]
pub type CalcDoneR = crate::BitReader;
#[doc = "Field `calc_done` writer - Interrupt raised when calculation is done"]
pub type CalcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `layer_cfg_almost_empty` reader - Interrupt raised when layer arguments FIFO almost empty"]
pub type LayerCfgAlmostEmptyR = crate::BitReader;
#[doc = "Field `layer_cfg_almost_empty` writer - Interrupt raised when layer arguments FIFO almost empty"]
pub type LayerCfgAlmostEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `layer_cfg_almost_full` reader - Interrupt raised when layer arguments FIFO almost full"]
pub type LayerCfgAlmostFullR = crate::BitReader;
#[doc = "Field `layer_cfg_almost_full` writer - Interrupt raised when layer arguments FIFO almost full"]
pub type LayerCfgAlmostFullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt raised when calculation is done"]
    #[inline(always)]
    pub fn calc_done(&self) -> CalcDoneR {
        CalcDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt raised when layer arguments FIFO almost empty"]
    #[inline(always)]
    pub fn layer_cfg_almost_empty(&self) -> LayerCfgAlmostEmptyR {
        LayerCfgAlmostEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt raised when layer arguments FIFO almost full"]
    #[inline(always)]
    pub fn layer_cfg_almost_full(&self) -> LayerCfgAlmostFullR {
        LayerCfgAlmostFullR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt raised when calculation is done"]
    #[inline(always)]
    #[must_use]
    pub fn calc_done(&mut self) -> CalcDoneW<InterruptStatusSpec> {
        CalcDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt raised when layer arguments FIFO almost empty"]
    #[inline(always)]
    #[must_use]
    pub fn layer_cfg_almost_empty(&mut self) -> LayerCfgAlmostEmptyW<InterruptStatusSpec> {
        LayerCfgAlmostEmptyW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt raised when layer arguments FIFO almost full"]
    #[inline(always)]
    #[must_use]
    pub fn layer_cfg_almost_full(&mut self) -> LayerCfgAlmostFullW<InterruptStatusSpec> {
        LayerCfgAlmostFullW::new(self, 2)
    }
}
#[doc = "Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptStatusSpec;
impl crate::RegisterSpec for InterruptStatusSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`interrupt_status::R`](R) reader structure"]
impl crate::Readable for InterruptStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_status::W`](W) writer structure"]
impl crate::Writable for InterruptStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets interrupt_status to value 0"]
impl crate::Resettable for InterruptStatusSpec {
    const RESET_VALUE: u64 = 0;
}
