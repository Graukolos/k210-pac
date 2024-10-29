#[doc = "Register `intr_mask` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `intr_mask` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `rx_under` reader - RX_UNDER"]
pub type RxUnderR = crate::BitReader;
#[doc = "Field `rx_under` writer - RX_UNDER"]
pub type RxUnderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_over` reader - RX_OVER"]
pub type RxOverR = crate::BitReader;
#[doc = "Field `rx_over` writer - RX_OVER"]
pub type RxOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_full` reader - RX_FULL"]
pub type RxFullR = crate::BitReader;
#[doc = "Field `rx_full` writer - RX_FULL"]
pub type RxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_over` reader - TX_OVER"]
pub type TxOverR = crate::BitReader;
#[doc = "Field `tx_over` writer - TX_OVER"]
pub type TxOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_empty` reader - TX_EMPTY"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `tx_empty` writer - TX_EMPTY"]
pub type TxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_req` reader - RD_REQ"]
pub type RdReqR = crate::BitReader;
#[doc = "Field `rd_req` writer - RD_REQ"]
pub type RdReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_abrt` reader - TX_ABRT"]
pub type TxAbrtR = crate::BitReader;
#[doc = "Field `tx_abrt` writer - TX_ABRT"]
pub type TxAbrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_done` reader - RX_DONE"]
pub type RxDoneR = crate::BitReader;
#[doc = "Field `rx_done` writer - RX_DONE"]
pub type RxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `activity` reader - ACTIVITY"]
pub type ActivityR = crate::BitReader;
#[doc = "Field `activity` writer - ACTIVITY"]
pub type ActivityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stop_det` reader - STOP_DET"]
pub type StopDetR = crate::BitReader;
#[doc = "Field `stop_det` writer - STOP_DET"]
pub type StopDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `start_det` reader - START_DET"]
pub type StartDetR = crate::BitReader;
#[doc = "Field `start_det` writer - START_DET"]
pub type StartDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gen_call` reader - GEN_CALL"]
pub type GenCallR = crate::BitReader;
#[doc = "Field `gen_call` writer - GEN_CALL"]
pub type GenCallW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX_UNDER"]
    #[inline(always)]
    pub fn rx_under(&self) -> RxUnderR {
        RxUnderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX_OVER"]
    #[inline(always)]
    pub fn rx_over(&self) -> RxOverR {
        RxOverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX_FULL"]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX_OVER"]
    #[inline(always)]
    pub fn tx_over(&self) -> TxOverR {
        TxOverR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX_EMPTY"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RD_REQ"]
    #[inline(always)]
    pub fn rd_req(&self) -> RdReqR {
        RdReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX_ABRT"]
    #[inline(always)]
    pub fn tx_abrt(&self) -> TxAbrtR {
        TxAbrtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX_DONE"]
    #[inline(always)]
    pub fn rx_done(&self) -> RxDoneR {
        RxDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ACTIVITY"]
    #[inline(always)]
    pub fn activity(&self) -> ActivityR {
        ActivityR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STOP_DET"]
    #[inline(always)]
    pub fn stop_det(&self) -> StopDetR {
        StopDetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - START_DET"]
    #[inline(always)]
    pub fn start_det(&self) -> StartDetR {
        StartDetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GEN_CALL"]
    #[inline(always)]
    pub fn gen_call(&self) -> GenCallR {
        GenCallR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX_UNDER"]
    #[inline(always)]
    #[must_use]
    pub fn rx_under(&mut self) -> RxUnderW<IntrMaskSpec> {
        RxUnderW::new(self, 0)
    }
    #[doc = "Bit 1 - RX_OVER"]
    #[inline(always)]
    #[must_use]
    pub fn rx_over(&mut self) -> RxOverW<IntrMaskSpec> {
        RxOverW::new(self, 1)
    }
    #[doc = "Bit 2 - RX_FULL"]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RxFullW<IntrMaskSpec> {
        RxFullW::new(self, 2)
    }
    #[doc = "Bit 3 - TX_OVER"]
    #[inline(always)]
    #[must_use]
    pub fn tx_over(&mut self) -> TxOverW<IntrMaskSpec> {
        TxOverW::new(self, 3)
    }
    #[doc = "Bit 4 - TX_EMPTY"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TxEmptyW<IntrMaskSpec> {
        TxEmptyW::new(self, 4)
    }
    #[doc = "Bit 5 - RD_REQ"]
    #[inline(always)]
    #[must_use]
    pub fn rd_req(&mut self) -> RdReqW<IntrMaskSpec> {
        RdReqW::new(self, 5)
    }
    #[doc = "Bit 6 - TX_ABRT"]
    #[inline(always)]
    #[must_use]
    pub fn tx_abrt(&mut self) -> TxAbrtW<IntrMaskSpec> {
        TxAbrtW::new(self, 6)
    }
    #[doc = "Bit 7 - RX_DONE"]
    #[inline(always)]
    #[must_use]
    pub fn rx_done(&mut self) -> RxDoneW<IntrMaskSpec> {
        RxDoneW::new(self, 7)
    }
    #[doc = "Bit 8 - ACTIVITY"]
    #[inline(always)]
    #[must_use]
    pub fn activity(&mut self) -> ActivityW<IntrMaskSpec> {
        ActivityW::new(self, 8)
    }
    #[doc = "Bit 9 - STOP_DET"]
    #[inline(always)]
    #[must_use]
    pub fn stop_det(&mut self) -> StopDetW<IntrMaskSpec> {
        StopDetW::new(self, 9)
    }
    #[doc = "Bit 10 - START_DET"]
    #[inline(always)]
    #[must_use]
    pub fn start_det(&mut self) -> StartDetW<IntrMaskSpec> {
        StartDetW::new(self, 10)
    }
    #[doc = "Bit 11 - GEN_CALL"]
    #[inline(always)]
    #[must_use]
    pub fn gen_call(&mut self) -> GenCallW<IntrMaskSpec> {
        GenCallW::new(self, 11)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMaskSpec;
impl crate::RegisterSpec for IntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for IntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for IntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets intr_mask to value 0"]
impl crate::Resettable for IntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
