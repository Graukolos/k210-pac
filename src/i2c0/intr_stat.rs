#[doc = "Register `intr_stat` reader"]
pub type R = crate::R<IntrStatSpec>;
#[doc = "Field `rx_under` reader - RX_UNDER"]
pub type RxUnderR = crate::BitReader;
#[doc = "Field `rx_over` reader - RX_OVER"]
pub type RxOverR = crate::BitReader;
#[doc = "Field `rx_full` reader - RX_FULL"]
pub type RxFullR = crate::BitReader;
#[doc = "Field `tx_over` reader - TX_OVER"]
pub type TxOverR = crate::BitReader;
#[doc = "Field `tx_empty` reader - TX_EMPTY"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `rd_req` reader - RD_REQ"]
pub type RdReqR = crate::BitReader;
#[doc = "Field `tx_abrt` reader - TX_ABRT"]
pub type TxAbrtR = crate::BitReader;
#[doc = "Field `rx_done` reader - RX_DONE"]
pub type RxDoneR = crate::BitReader;
#[doc = "Field `activity` reader - ACTIVITY"]
pub type ActivityR = crate::BitReader;
#[doc = "Field `stop_det` reader - STOP_DET"]
pub type StopDetR = crate::BitReader;
#[doc = "Field `start_det` reader - START_DET"]
pub type StartDetR = crate::BitReader;
#[doc = "Field `gen_call` reader - GEN_CALL"]
pub type GenCallR = crate::BitReader;
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
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrStatSpec;
impl crate::RegisterSpec for IntrStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_stat::R`](R) reader structure"]
impl crate::Readable for IntrStatSpec {}
#[doc = "`reset()` method sets intr_stat to value 0"]
impl crate::Resettable for IntrStatSpec {
    const RESET_VALUE: u32 = 0;
}
