#[doc = "Register `dma_sel0` reader"]
pub type R = crate::R<DmaSel0Spec>;
#[doc = "Register `dma_sel0` writer"]
pub type W = crate::W<DmaSel0Spec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmaselect {
    #[doc = "0: `0`"]
    Ssi0RxReq = 0,
    #[doc = "1: `1`"]
    Ssi0TxReq = 1,
    #[doc = "2: `10`"]
    Ssi1RxReq = 2,
    #[doc = "3: `11`"]
    Ssi1TxReq = 3,
    #[doc = "4: `100`"]
    Ssi2RxReq = 4,
    #[doc = "5: `101`"]
    Ssi2TxReq = 5,
    #[doc = "6: `110`"]
    Ssi3RxReq = 6,
    #[doc = "7: `111`"]
    Ssi3TxReq = 7,
    #[doc = "8: `1000`"]
    I2c0RxReq = 8,
    #[doc = "9: `1001`"]
    I2c0TxReq = 9,
    #[doc = "10: `1010`"]
    I2c1RxReq = 10,
    #[doc = "11: `1011`"]
    I2c1TxReq = 11,
    #[doc = "12: `1100`"]
    I2c2RxReq = 12,
    #[doc = "13: `1101`"]
    I2c2TxReq = 13,
    #[doc = "14: `1110`"]
    Uart1RxReq = 14,
    #[doc = "15: `1111`"]
    Uart1TxReq = 15,
    #[doc = "16: `10000`"]
    Uart2RxReq = 16,
    #[doc = "17: `10001`"]
    Uart2TxReq = 17,
    #[doc = "18: `10010`"]
    Uart3RxReq = 18,
    #[doc = "19: `10011`"]
    Uart3TxReq = 19,
    #[doc = "20: `10100`"]
    AesReq = 20,
    #[doc = "21: `10101`"]
    ShaRxReq = 21,
    #[doc = "22: `10110`"]
    AiRxReq = 22,
    #[doc = "23: `10111`"]
    FftRxReq = 23,
    #[doc = "24: `11000`"]
    FftTxReq = 24,
    #[doc = "25: `11001`"]
    I2s0TxReq = 25,
    #[doc = "26: `11010`"]
    I2s0RxReq = 26,
    #[doc = "27: `11011`"]
    I2s1TxReq = 27,
    #[doc = "28: `11100`"]
    I2s1RxReq = 28,
    #[doc = "29: `11101`"]
    I2s2TxReq = 29,
    #[doc = "30: `11110`"]
    I2s2RxReq = 30,
    #[doc = "31: `11111`"]
    I2s0BfDirReq = 31,
    #[doc = "32: `100000`"]
    I2s0BfVoiceReq = 32,
}
impl From<Dmaselect> for u8 {
    #[inline(always)]
    fn from(variant: Dmaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmaselect {
    type Ux = u8;
}
impl crate::IsEnum for Dmaselect {}
#[doc = "Field `dma_sel0` reader - "]
pub type DmaSel0R = crate::FieldReader<Dmaselect>;
impl DmaSel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmaselect> {
        match self.bits {
            0 => Some(Dmaselect::Ssi0RxReq),
            1 => Some(Dmaselect::Ssi0TxReq),
            2 => Some(Dmaselect::Ssi1RxReq),
            3 => Some(Dmaselect::Ssi1TxReq),
            4 => Some(Dmaselect::Ssi2RxReq),
            5 => Some(Dmaselect::Ssi2TxReq),
            6 => Some(Dmaselect::Ssi3RxReq),
            7 => Some(Dmaselect::Ssi3TxReq),
            8 => Some(Dmaselect::I2c0RxReq),
            9 => Some(Dmaselect::I2c0TxReq),
            10 => Some(Dmaselect::I2c1RxReq),
            11 => Some(Dmaselect::I2c1TxReq),
            12 => Some(Dmaselect::I2c2RxReq),
            13 => Some(Dmaselect::I2c2TxReq),
            14 => Some(Dmaselect::Uart1RxReq),
            15 => Some(Dmaselect::Uart1TxReq),
            16 => Some(Dmaselect::Uart2RxReq),
            17 => Some(Dmaselect::Uart2TxReq),
            18 => Some(Dmaselect::Uart3RxReq),
            19 => Some(Dmaselect::Uart3TxReq),
            20 => Some(Dmaselect::AesReq),
            21 => Some(Dmaselect::ShaRxReq),
            22 => Some(Dmaselect::AiRxReq),
            23 => Some(Dmaselect::FftRxReq),
            24 => Some(Dmaselect::FftTxReq),
            25 => Some(Dmaselect::I2s0TxReq),
            26 => Some(Dmaselect::I2s0RxReq),
            27 => Some(Dmaselect::I2s1TxReq),
            28 => Some(Dmaselect::I2s1RxReq),
            29 => Some(Dmaselect::I2s2TxReq),
            30 => Some(Dmaselect::I2s2RxReq),
            31 => Some(Dmaselect::I2s0BfDirReq),
            32 => Some(Dmaselect::I2s0BfVoiceReq),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_ssi0_rx_req(&self) -> bool {
        *self == Dmaselect::Ssi0RxReq
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ssi0_tx_req(&self) -> bool {
        *self == Dmaselect::Ssi0TxReq
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ssi1_rx_req(&self) -> bool {
        *self == Dmaselect::Ssi1RxReq
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_ssi1_tx_req(&self) -> bool {
        *self == Dmaselect::Ssi1TxReq
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_ssi2_rx_req(&self) -> bool {
        *self == Dmaselect::Ssi2RxReq
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_ssi2_tx_req(&self) -> bool {
        *self == Dmaselect::Ssi2TxReq
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_ssi3_rx_req(&self) -> bool {
        *self == Dmaselect::Ssi3RxReq
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_ssi3_tx_req(&self) -> bool {
        *self == Dmaselect::Ssi3TxReq
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_i2c0_rx_req(&self) -> bool {
        *self == Dmaselect::I2c0RxReq
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_i2c0_tx_req(&self) -> bool {
        *self == Dmaselect::I2c0TxReq
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_i2c1_rx_req(&self) -> bool {
        *self == Dmaselect::I2c1RxReq
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_i2c1_tx_req(&self) -> bool {
        *self == Dmaselect::I2c1TxReq
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_i2c2_rx_req(&self) -> bool {
        *self == Dmaselect::I2c2RxReq
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_i2c2_tx_req(&self) -> bool {
        *self == Dmaselect::I2c2TxReq
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_uart1_rx_req(&self) -> bool {
        *self == Dmaselect::Uart1RxReq
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_uart1_tx_req(&self) -> bool {
        *self == Dmaselect::Uart1TxReq
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_uart2_rx_req(&self) -> bool {
        *self == Dmaselect::Uart2RxReq
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_uart2_tx_req(&self) -> bool {
        *self == Dmaselect::Uart2TxReq
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_uart3_rx_req(&self) -> bool {
        *self == Dmaselect::Uart3RxReq
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_uart3_tx_req(&self) -> bool {
        *self == Dmaselect::Uart3TxReq
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn is_aes_req(&self) -> bool {
        *self == Dmaselect::AesReq
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn is_sha_rx_req(&self) -> bool {
        *self == Dmaselect::ShaRxReq
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn is_ai_rx_req(&self) -> bool {
        *self == Dmaselect::AiRxReq
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn is_fft_rx_req(&self) -> bool {
        *self == Dmaselect::FftRxReq
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn is_fft_tx_req(&self) -> bool {
        *self == Dmaselect::FftTxReq
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn is_i2s0_tx_req(&self) -> bool {
        *self == Dmaselect::I2s0TxReq
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn is_i2s0_rx_req(&self) -> bool {
        *self == Dmaselect::I2s0RxReq
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn is_i2s1_tx_req(&self) -> bool {
        *self == Dmaselect::I2s1TxReq
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn is_i2s1_rx_req(&self) -> bool {
        *self == Dmaselect::I2s1RxReq
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn is_i2s2_tx_req(&self) -> bool {
        *self == Dmaselect::I2s2TxReq
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn is_i2s2_rx_req(&self) -> bool {
        *self == Dmaselect::I2s2RxReq
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn is_i2s0_bf_dir_req(&self) -> bool {
        *self == Dmaselect::I2s0BfDirReq
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn is_i2s0_bf_voice_req(&self) -> bool {
        *self == Dmaselect::I2s0BfVoiceReq
    }
}
#[doc = "Field `dma_sel0` writer - "]
pub type DmaSel0W<'a, REG> = crate::FieldWriter<'a, REG, 6, Dmaselect>;
impl<'a, REG> DmaSel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ssi0_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Ssi0RxReq)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ssi0_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Ssi0TxReq)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ssi1_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Ssi1RxReq)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ssi1_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Ssi1TxReq)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ssi2_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Ssi2RxReq)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ssi2_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Ssi2TxReq)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ssi3_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Ssi3RxReq)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn ssi3_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Ssi3TxReq)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn i2c0_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2c0RxReq)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn i2c0_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2c0TxReq)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn i2c1_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2c1RxReq)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn i2c1_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2c1TxReq)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn i2c2_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2c2RxReq)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn i2c2_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2c2TxReq)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn uart1_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Uart1RxReq)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn uart1_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Uart1TxReq)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn uart2_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Uart2RxReq)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn uart2_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Uart2TxReq)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn uart3_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Uart3RxReq)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn uart3_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Uart3TxReq)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn aes_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::AesReq)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn sha_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::ShaRxReq)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn ai_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::AiRxReq)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn fft_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::FftRxReq)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn fft_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::FftTxReq)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn i2s0_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2s0TxReq)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn i2s0_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2s0RxReq)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn i2s1_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2s1TxReq)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn i2s1_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2s1RxReq)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn i2s2_tx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2s2TxReq)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn i2s2_rx_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2s2RxReq)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn i2s0_bf_dir_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2s0BfDirReq)
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn i2s0_bf_voice_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::I2s0BfVoiceReq)
    }
}
#[doc = "Field `dma_sel1` reader - "]
pub use DmaSel0R as DmaSel1R;
#[doc = "Field `dma_sel2` reader - "]
pub use DmaSel0R as DmaSel2R;
#[doc = "Field `dma_sel3` reader - "]
pub use DmaSel0R as DmaSel3R;
#[doc = "Field `dma_sel4` reader - "]
pub use DmaSel0R as DmaSel4R;
#[doc = "Field `dma_sel1` writer - "]
pub use DmaSel0W as DmaSel1W;
#[doc = "Field `dma_sel2` writer - "]
pub use DmaSel0W as DmaSel2W;
#[doc = "Field `dma_sel3` writer - "]
pub use DmaSel0W as DmaSel3W;
#[doc = "Field `dma_sel4` writer - "]
pub use DmaSel0W as DmaSel4W;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn dma_sel0(&self) -> DmaSel0R {
        DmaSel0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn dma_sel1(&self) -> DmaSel1R {
        DmaSel1R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn dma_sel2(&self) -> DmaSel2R {
        DmaSel2R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn dma_sel3(&self) -> DmaSel3R {
        DmaSel3R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn dma_sel4(&self) -> DmaSel4R {
        DmaSel4R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn dma_sel0(&mut self) -> DmaSel0W<DmaSel0Spec> {
        DmaSel0W::new(self, 0)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    #[must_use]
    pub fn dma_sel1(&mut self) -> DmaSel1W<DmaSel0Spec> {
        DmaSel1W::new(self, 6)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    #[must_use]
    pub fn dma_sel2(&mut self) -> DmaSel2W<DmaSel0Spec> {
        DmaSel2W::new(self, 12)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    #[must_use]
    pub fn dma_sel3(&mut self) -> DmaSel3W<DmaSel0Spec> {
        DmaSel3W::new(self, 18)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    #[must_use]
    pub fn dma_sel4(&mut self) -> DmaSel4W<DmaSel0Spec> {
        DmaSel4W::new(self, 24)
    }
}
#[doc = "DMA handshake selector\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSel0Spec;
impl crate::RegisterSpec for DmaSel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_sel0::R`](R) reader structure"]
impl crate::Readable for DmaSel0Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_sel0::W`](W) writer structure"]
impl crate::Writable for DmaSel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dma_sel0 to value 0"]
impl crate::Resettable for DmaSel0Spec {
    const RESET_VALUE: u32 = 0;
}
