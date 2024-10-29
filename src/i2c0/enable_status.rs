#[doc = "Register `enable_status` reader"]
pub type R = crate::R<EnableStatusSpec>;
#[doc = "Field `ic_enable` reader - IC_ENABLE"]
pub type IcEnableR = crate::BitReader;
#[doc = "Field `slv_dis_busy` reader - SLV_DIS_BUSY"]
pub type SlvDisBusyR = crate::BitReader;
#[doc = "Field `slv_rx_data_lost` reader - SLV_RX_DATA_LOST"]
pub type SlvRxDataLostR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IC_ENABLE"]
    #[inline(always)]
    pub fn ic_enable(&self) -> IcEnableR {
        IcEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SLV_DIS_BUSY"]
    #[inline(always)]
    pub fn slv_dis_busy(&self) -> SlvDisBusyR {
        SlvDisBusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SLV_RX_DATA_LOST"]
    #[inline(always)]
    pub fn slv_rx_data_lost(&self) -> SlvRxDataLostR {
        SlvRxDataLostR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Enable Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableStatusSpec;
impl crate::RegisterSpec for EnableStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_status::R`](R) reader structure"]
impl crate::Readable for EnableStatusSpec {}
#[doc = "`reset()` method sets enable_status to value 0"]
impl crate::Resettable for EnableStatusSpec {
    const RESET_VALUE: u32 = 0;
}
