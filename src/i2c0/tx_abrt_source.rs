#[doc = "Register `tx_abrt_source` reader"]
pub type R = crate::R<TxAbrtSourceSpec>;
#[doc = "Register `tx_abrt_source` writer"]
pub type W = crate::W<TxAbrtSourceSpec>;
#[doc = "Field `addr7_noack` reader - 7B_ADDR_NOACK"]
pub type Addr7NoackR = crate::BitReader;
#[doc = "Field `addr7_noack` writer - 7B_ADDR_NOACK"]
pub type Addr7NoackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `addr1_10_noack` reader - 10B_ADDR1_NOACK"]
pub type Addr1_10NoackR = crate::BitReader;
#[doc = "Field `addr1_10_noack` writer - 10B_ADDR1_NOACK"]
pub type Addr1_10NoackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `addr2_10_noack` reader - 10B_ADDR2_NOACK"]
pub type Addr2_10NoackR = crate::BitReader;
#[doc = "Field `addr2_10_noack` writer - 10B_ADDR2_NOACK"]
pub type Addr2_10NoackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txdata_noack` reader - TXDATA_NOACK"]
pub type TxdataNoackR = crate::BitReader;
#[doc = "Field `txdata_noack` writer - TXDATA_NOACK"]
pub type TxdataNoackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gcall_noack` reader - GCALL_NOACK"]
pub type GcallNoackR = crate::BitReader;
#[doc = "Field `gcall_noack` writer - GCALL_NOACK"]
pub type GcallNoackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gcall_read` reader - GCALL_READ"]
pub type GcallReadR = crate::BitReader;
#[doc = "Field `gcall_read` writer - GCALL_READ"]
pub type GcallReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hs_ackdet` reader - HS_ACKDET"]
pub type HsAckdetR = crate::BitReader;
#[doc = "Field `hs_ackdet` writer - HS_ACKDET"]
pub type HsAckdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sbyte_ackdet` reader - SBYTE_ACKDET"]
pub type SbyteAckdetR = crate::BitReader;
#[doc = "Field `sbyte_ackdet` writer - SBYTE_ACKDET"]
pub type SbyteAckdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hs_norstrt` reader - HS_NORSTRT"]
pub type HsNorstrtR = crate::BitReader;
#[doc = "Field `hs_norstrt` writer - HS_NORSTRT"]
pub type HsNorstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sbyte_norstrt` reader - SBYTE_NORSTRT"]
pub type SbyteNorstrtR = crate::BitReader;
#[doc = "Field `sbyte_norstrt` writer - SBYTE_NORSTRT"]
pub type SbyteNorstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_10_norstrt` reader - 10B_RD_NORSTRT"]
pub type Rd10NorstrtR = crate::BitReader;
#[doc = "Field `rd_10_norstrt` writer - 10B_RD_NORSTRT"]
pub type Rd10NorstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `master_dis` reader - MASTER_DIS"]
pub type MasterDisR = crate::BitReader;
#[doc = "Field `master_dis` writer - MASTER_DIS"]
pub type MasterDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mst_arblost` reader - MST_ARBLOST"]
pub type MstArblostR = crate::BitReader;
#[doc = "Field `mst_arblost` writer - MST_ARBLOST"]
pub type MstArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvflush_txfifo` reader - SLVFLUSH_TXFIFO"]
pub type SlvflushTxfifoR = crate::BitReader;
#[doc = "Field `slvflush_txfifo` writer - SLVFLUSH_TXFIFO"]
pub type SlvflushTxfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slv_arblost` reader - SLV_ARBLOST"]
pub type SlvArblostR = crate::BitReader;
#[doc = "Field `slv_arblost` writer - SLV_ARBLOST"]
pub type SlvArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvrd_intx` reader - SLVRD_INTX"]
pub type SlvrdIntxR = crate::BitReader;
#[doc = "Field `slvrd_intx` writer - SLVRD_INTX"]
pub type SlvrdIntxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `user_abrt` reader - USER_ABRT"]
pub type UserAbrtR = crate::BitReader;
#[doc = "Field `user_abrt` writer - USER_ABRT"]
pub type UserAbrtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 7B_ADDR_NOACK"]
    #[inline(always)]
    pub fn addr7_noack(&self) -> Addr7NoackR {
        Addr7NoackR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 10B_ADDR1_NOACK"]
    #[inline(always)]
    pub fn addr1_10_noack(&self) -> Addr1_10NoackR {
        Addr1_10NoackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 10B_ADDR2_NOACK"]
    #[inline(always)]
    pub fn addr2_10_noack(&self) -> Addr2_10NoackR {
        Addr2_10NoackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXDATA_NOACK"]
    #[inline(always)]
    pub fn txdata_noack(&self) -> TxdataNoackR {
        TxdataNoackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GCALL_NOACK"]
    #[inline(always)]
    pub fn gcall_noack(&self) -> GcallNoackR {
        GcallNoackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GCALL_READ"]
    #[inline(always)]
    pub fn gcall_read(&self) -> GcallReadR {
        GcallReadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HS_ACKDET"]
    #[inline(always)]
    pub fn hs_ackdet(&self) -> HsAckdetR {
        HsAckdetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SBYTE_ACKDET"]
    #[inline(always)]
    pub fn sbyte_ackdet(&self) -> SbyteAckdetR {
        SbyteAckdetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HS_NORSTRT"]
    #[inline(always)]
    pub fn hs_norstrt(&self) -> HsNorstrtR {
        HsNorstrtR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SBYTE_NORSTRT"]
    #[inline(always)]
    pub fn sbyte_norstrt(&self) -> SbyteNorstrtR {
        SbyteNorstrtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10B_RD_NORSTRT"]
    #[inline(always)]
    pub fn rd_10_norstrt(&self) -> Rd10NorstrtR {
        Rd10NorstrtR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MASTER_DIS"]
    #[inline(always)]
    pub fn master_dis(&self) -> MasterDisR {
        MasterDisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MST_ARBLOST"]
    #[inline(always)]
    pub fn mst_arblost(&self) -> MstArblostR {
        MstArblostR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SLVFLUSH_TXFIFO"]
    #[inline(always)]
    pub fn slvflush_txfifo(&self) -> SlvflushTxfifoR {
        SlvflushTxfifoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SLV_ARBLOST"]
    #[inline(always)]
    pub fn slv_arblost(&self) -> SlvArblostR {
        SlvArblostR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SLVRD_INTX"]
    #[inline(always)]
    pub fn slvrd_intx(&self) -> SlvrdIntxR {
        SlvrdIntxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - USER_ABRT"]
    #[inline(always)]
    pub fn user_abrt(&self) -> UserAbrtR {
        UserAbrtR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 7B_ADDR_NOACK"]
    #[inline(always)]
    #[must_use]
    pub fn addr7_noack(&mut self) -> Addr7NoackW<TxAbrtSourceSpec> {
        Addr7NoackW::new(self, 0)
    }
    #[doc = "Bit 1 - 10B_ADDR1_NOACK"]
    #[inline(always)]
    #[must_use]
    pub fn addr1_10_noack(&mut self) -> Addr1_10NoackW<TxAbrtSourceSpec> {
        Addr1_10NoackW::new(self, 1)
    }
    #[doc = "Bit 2 - 10B_ADDR2_NOACK"]
    #[inline(always)]
    #[must_use]
    pub fn addr2_10_noack(&mut self) -> Addr2_10NoackW<TxAbrtSourceSpec> {
        Addr2_10NoackW::new(self, 2)
    }
    #[doc = "Bit 3 - TXDATA_NOACK"]
    #[inline(always)]
    #[must_use]
    pub fn txdata_noack(&mut self) -> TxdataNoackW<TxAbrtSourceSpec> {
        TxdataNoackW::new(self, 3)
    }
    #[doc = "Bit 4 - GCALL_NOACK"]
    #[inline(always)]
    #[must_use]
    pub fn gcall_noack(&mut self) -> GcallNoackW<TxAbrtSourceSpec> {
        GcallNoackW::new(self, 4)
    }
    #[doc = "Bit 5 - GCALL_READ"]
    #[inline(always)]
    #[must_use]
    pub fn gcall_read(&mut self) -> GcallReadW<TxAbrtSourceSpec> {
        GcallReadW::new(self, 5)
    }
    #[doc = "Bit 6 - HS_ACKDET"]
    #[inline(always)]
    #[must_use]
    pub fn hs_ackdet(&mut self) -> HsAckdetW<TxAbrtSourceSpec> {
        HsAckdetW::new(self, 6)
    }
    #[doc = "Bit 7 - SBYTE_ACKDET"]
    #[inline(always)]
    #[must_use]
    pub fn sbyte_ackdet(&mut self) -> SbyteAckdetW<TxAbrtSourceSpec> {
        SbyteAckdetW::new(self, 7)
    }
    #[doc = "Bit 8 - HS_NORSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn hs_norstrt(&mut self) -> HsNorstrtW<TxAbrtSourceSpec> {
        HsNorstrtW::new(self, 8)
    }
    #[doc = "Bit 9 - SBYTE_NORSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn sbyte_norstrt(&mut self) -> SbyteNorstrtW<TxAbrtSourceSpec> {
        SbyteNorstrtW::new(self, 9)
    }
    #[doc = "Bit 10 - 10B_RD_NORSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn rd_10_norstrt(&mut self) -> Rd10NorstrtW<TxAbrtSourceSpec> {
        Rd10NorstrtW::new(self, 10)
    }
    #[doc = "Bit 11 - MASTER_DIS"]
    #[inline(always)]
    #[must_use]
    pub fn master_dis(&mut self) -> MasterDisW<TxAbrtSourceSpec> {
        MasterDisW::new(self, 11)
    }
    #[doc = "Bit 12 - MST_ARBLOST"]
    #[inline(always)]
    #[must_use]
    pub fn mst_arblost(&mut self) -> MstArblostW<TxAbrtSourceSpec> {
        MstArblostW::new(self, 12)
    }
    #[doc = "Bit 13 - SLVFLUSH_TXFIFO"]
    #[inline(always)]
    #[must_use]
    pub fn slvflush_txfifo(&mut self) -> SlvflushTxfifoW<TxAbrtSourceSpec> {
        SlvflushTxfifoW::new(self, 13)
    }
    #[doc = "Bit 14 - SLV_ARBLOST"]
    #[inline(always)]
    #[must_use]
    pub fn slv_arblost(&mut self) -> SlvArblostW<TxAbrtSourceSpec> {
        SlvArblostW::new(self, 14)
    }
    #[doc = "Bit 15 - SLVRD_INTX"]
    #[inline(always)]
    #[must_use]
    pub fn slvrd_intx(&mut self) -> SlvrdIntxW<TxAbrtSourceSpec> {
        SlvrdIntxW::new(self, 15)
    }
    #[doc = "Bit 16 - USER_ABRT"]
    #[inline(always)]
    #[must_use]
    pub fn user_abrt(&mut self) -> UserAbrtW<TxAbrtSourceSpec> {
        UserAbrtW::new(self, 16)
    }
}
#[doc = "Transmit Abort Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_abrt_source::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_abrt_source::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxAbrtSourceSpec;
impl crate::RegisterSpec for TxAbrtSourceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_abrt_source::R`](R) reader structure"]
impl crate::Readable for TxAbrtSourceSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_abrt_source::W`](W) writer structure"]
impl crate::Writable for TxAbrtSourceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tx_abrt_source to value 0"]
impl crate::Resettable for TxAbrtSourceSpec {
    const RESET_VALUE: u32 = 0;
}
