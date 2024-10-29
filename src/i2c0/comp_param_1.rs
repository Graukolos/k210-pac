#[doc = "Register `comp_param_1` reader"]
pub type R = crate::R<CompParam1Spec>;
#[doc = "Field `apb_data_width` reader - APB_DATA_WIDTH"]
pub type ApbDataWidthR = crate::FieldReader;
#[doc = "Field `max_speed_mode` reader - MAX_SPEED_MODE"]
pub type MaxSpeedModeR = crate::FieldReader;
#[doc = "Field `hc_count_values` reader - HC_COUNT_VALUES"]
pub type HcCountValuesR = crate::BitReader;
#[doc = "Field `intr_io` reader - INTR_IO"]
pub type IntrIoR = crate::BitReader;
#[doc = "Field `has_dma` reader - HAS_DMA"]
pub type HasDmaR = crate::BitReader;
#[doc = "Field `encoded_params` reader - ENCODED_PARAMS"]
pub type EncodedParamsR = crate::BitReader;
#[doc = "Field `rx_buffer_depth` reader - RX_BUFFER_DEPTH"]
pub type RxBufferDepthR = crate::FieldReader;
#[doc = "Field `tx_buffer_depth` reader - TX_BUFFER_DEPTH"]
pub type TxBufferDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - APB_DATA_WIDTH"]
    #[inline(always)]
    pub fn apb_data_width(&self) -> ApbDataWidthR {
        ApbDataWidthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - MAX_SPEED_MODE"]
    #[inline(always)]
    pub fn max_speed_mode(&self) -> MaxSpeedModeR {
        MaxSpeedModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - HC_COUNT_VALUES"]
    #[inline(always)]
    pub fn hc_count_values(&self) -> HcCountValuesR {
        HcCountValuesR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - INTR_IO"]
    #[inline(always)]
    pub fn intr_io(&self) -> IntrIoR {
        IntrIoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HAS_DMA"]
    #[inline(always)]
    pub fn has_dma(&self) -> HasDmaR {
        HasDmaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ENCODED_PARAMS"]
    #[inline(always)]
    pub fn encoded_params(&self) -> EncodedParamsR {
        EncodedParamsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - RX_BUFFER_DEPTH"]
    #[inline(always)]
    pub fn rx_buffer_depth(&self) -> RxBufferDepthR {
        RxBufferDepthR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TX_BUFFER_DEPTH"]
    #[inline(always)]
    pub fn tx_buffer_depth(&self) -> TxBufferDepthR {
        TxBufferDepthR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Component Parameter Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_param_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompParam1Spec;
impl crate::RegisterSpec for CompParam1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_param_1::R`](R) reader structure"]
impl crate::Readable for CompParam1Spec {}
#[doc = "`reset()` method sets comp_param_1 to value 0"]
impl crate::Resettable for CompParam1Spec {
    const RESET_VALUE: u32 = 0;
}
