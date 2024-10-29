#[doc = "Register `cfg` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `cfg` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Source multi-block transfer type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MultiblkTransferType {
    #[doc = "0: Continuous multi-block type"]
    Contiguous = 0,
    #[doc = "1: Reload multi-block type"]
    Reload = 1,
    #[doc = "2: Shadow register based multi-block type"]
    ShadowRegister = 2,
    #[doc = "3: Linked list based multi-block type"]
    LinkedList = 3,
}
impl From<MultiblkTransferType> for u8 {
    #[inline(always)]
    fn from(variant: MultiblkTransferType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MultiblkTransferType {
    type Ux = u8;
}
impl crate::IsEnum for MultiblkTransferType {}
#[doc = "Field `src_multblk_type` reader - Source multi-block transfer type"]
pub type SrcMultblkTypeR = crate::FieldReader<MultiblkTransferType>;
impl SrcMultblkTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MultiblkTransferType {
        match self.bits {
            0 => MultiblkTransferType::Contiguous,
            1 => MultiblkTransferType::Reload,
            2 => MultiblkTransferType::ShadowRegister,
            3 => MultiblkTransferType::LinkedList,
            _ => unreachable!(),
        }
    }
    #[doc = "Continuous multi-block type"]
    #[inline(always)]
    pub fn is_contiguous(&self) -> bool {
        *self == MultiblkTransferType::Contiguous
    }
    #[doc = "Reload multi-block type"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == MultiblkTransferType::Reload
    }
    #[doc = "Shadow register based multi-block type"]
    #[inline(always)]
    pub fn is_shadow_register(&self) -> bool {
        *self == MultiblkTransferType::ShadowRegister
    }
    #[doc = "Linked list based multi-block type"]
    #[inline(always)]
    pub fn is_linked_list(&self) -> bool {
        *self == MultiblkTransferType::LinkedList
    }
}
#[doc = "Field `src_multblk_type` writer - Source multi-block transfer type"]
pub type SrcMultblkTypeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, MultiblkTransferType, crate::Safe>;
impl<'a, REG> SrcMultblkTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Continuous multi-block type"]
    #[inline(always)]
    pub fn contiguous(self) -> &'a mut crate::W<REG> {
        self.variant(MultiblkTransferType::Contiguous)
    }
    #[doc = "Reload multi-block type"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut crate::W<REG> {
        self.variant(MultiblkTransferType::Reload)
    }
    #[doc = "Shadow register based multi-block type"]
    #[inline(always)]
    pub fn shadow_register(self) -> &'a mut crate::W<REG> {
        self.variant(MultiblkTransferType::ShadowRegister)
    }
    #[doc = "Linked list based multi-block type"]
    #[inline(always)]
    pub fn linked_list(self) -> &'a mut crate::W<REG> {
        self.variant(MultiblkTransferType::LinkedList)
    }
}
#[doc = "Field `dst_multblk_type` reader - Destination multi-block transfer type"]
pub use SrcMultblkTypeR as DstMultblkTypeR;
#[doc = "Field `dst_multblk_type` writer - Destination multi-block transfer type"]
pub use SrcMultblkTypeW as DstMultblkTypeW;
#[doc = "Transfer type and flow control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TtFc {
    #[doc = "0: Transfer memory to memory and flow controller is DMAC"]
    Mem2memDma = 0,
    #[doc = "1: Transfer memory to peripheral and flow controller is DMAC"]
    Mem2prfDma = 1,
    #[doc = "2: Transfer peripheral to memory and flow controller is DMAC"]
    Prf2memDma = 2,
    #[doc = "3: Transfer peripheral to peripheral and flow controller is DMAC"]
    Prf2prfDma = 3,
    #[doc = "4: Transfer peripheral to memory and flow controller is source peripheral"]
    Prf2memPrf = 4,
    #[doc = "5: Transfer peripheral to peripheral and flow controller is source peripheral"]
    Prf2prfSrcprf = 5,
    #[doc = "6: Transfer memory to peripheral and flow controller is destination peripheral"]
    Mem2prfPrf = 6,
    #[doc = "7: Transfer peripheral to peripheral and flow controller is destination peripheral"]
    Prf2prfDstprf = 7,
}
impl From<TtFc> for u8 {
    #[inline(always)]
    fn from(variant: TtFc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TtFc {
    type Ux = u8;
}
impl crate::IsEnum for TtFc {}
#[doc = "Field `tt_fc` reader - Transfer type and flow control"]
pub type TtFcR = crate::FieldReader<TtFc>;
impl TtFcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TtFc {
        match self.bits {
            0 => TtFc::Mem2memDma,
            1 => TtFc::Mem2prfDma,
            2 => TtFc::Prf2memDma,
            3 => TtFc::Prf2prfDma,
            4 => TtFc::Prf2memPrf,
            5 => TtFc::Prf2prfSrcprf,
            6 => TtFc::Mem2prfPrf,
            7 => TtFc::Prf2prfDstprf,
            _ => unreachable!(),
        }
    }
    #[doc = "Transfer memory to memory and flow controller is DMAC"]
    #[inline(always)]
    pub fn is_mem2mem_dma(&self) -> bool {
        *self == TtFc::Mem2memDma
    }
    #[doc = "Transfer memory to peripheral and flow controller is DMAC"]
    #[inline(always)]
    pub fn is_mem2prf_dma(&self) -> bool {
        *self == TtFc::Mem2prfDma
    }
    #[doc = "Transfer peripheral to memory and flow controller is DMAC"]
    #[inline(always)]
    pub fn is_prf2mem_dma(&self) -> bool {
        *self == TtFc::Prf2memDma
    }
    #[doc = "Transfer peripheral to peripheral and flow controller is DMAC"]
    #[inline(always)]
    pub fn is_prf2prf_dma(&self) -> bool {
        *self == TtFc::Prf2prfDma
    }
    #[doc = "Transfer peripheral to memory and flow controller is source peripheral"]
    #[inline(always)]
    pub fn is_prf2mem_prf(&self) -> bool {
        *self == TtFc::Prf2memPrf
    }
    #[doc = "Transfer peripheral to peripheral and flow controller is source peripheral"]
    #[inline(always)]
    pub fn is_prf2prf_srcprf(&self) -> bool {
        *self == TtFc::Prf2prfSrcprf
    }
    #[doc = "Transfer memory to peripheral and flow controller is destination peripheral"]
    #[inline(always)]
    pub fn is_mem2prf_prf(&self) -> bool {
        *self == TtFc::Mem2prfPrf
    }
    #[doc = "Transfer peripheral to peripheral and flow controller is destination peripheral"]
    #[inline(always)]
    pub fn is_prf2prf_dstprf(&self) -> bool {
        *self == TtFc::Prf2prfDstprf
    }
}
#[doc = "Field `tt_fc` writer - Transfer type and flow control"]
pub type TtFcW<'a, REG> = crate::FieldWriter<'a, REG, 3, TtFc, crate::Safe>;
impl<'a, REG> TtFcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transfer memory to memory and flow controller is DMAC"]
    #[inline(always)]
    pub fn mem2mem_dma(self) -> &'a mut crate::W<REG> {
        self.variant(TtFc::Mem2memDma)
    }
    #[doc = "Transfer memory to peripheral and flow controller is DMAC"]
    #[inline(always)]
    pub fn mem2prf_dma(self) -> &'a mut crate::W<REG> {
        self.variant(TtFc::Mem2prfDma)
    }
    #[doc = "Transfer peripheral to memory and flow controller is DMAC"]
    #[inline(always)]
    pub fn prf2mem_dma(self) -> &'a mut crate::W<REG> {
        self.variant(TtFc::Prf2memDma)
    }
    #[doc = "Transfer peripheral to peripheral and flow controller is DMAC"]
    #[inline(always)]
    pub fn prf2prf_dma(self) -> &'a mut crate::W<REG> {
        self.variant(TtFc::Prf2prfDma)
    }
    #[doc = "Transfer peripheral to memory and flow controller is source peripheral"]
    #[inline(always)]
    pub fn prf2mem_prf(self) -> &'a mut crate::W<REG> {
        self.variant(TtFc::Prf2memPrf)
    }
    #[doc = "Transfer peripheral to peripheral and flow controller is source peripheral"]
    #[inline(always)]
    pub fn prf2prf_srcprf(self) -> &'a mut crate::W<REG> {
        self.variant(TtFc::Prf2prfSrcprf)
    }
    #[doc = "Transfer memory to peripheral and flow controller is destination peripheral"]
    #[inline(always)]
    pub fn mem2prf_prf(self) -> &'a mut crate::W<REG> {
        self.variant(TtFc::Mem2prfPrf)
    }
    #[doc = "Transfer peripheral to peripheral and flow controller is destination peripheral"]
    #[inline(always)]
    pub fn prf2prf_dstprf(self) -> &'a mut crate::W<REG> {
        self.variant(TtFc::Prf2prfDstprf)
    }
}
#[doc = "Source software or hardware handshaking select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handshaking {
    #[doc = "0: Hardware handshaking is used"]
    Hardware = 0,
    #[doc = "1: Software handshaking is used"]
    Software = 1,
}
impl From<Handshaking> for bool {
    #[inline(always)]
    fn from(variant: Handshaking) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hs_sel_src` reader - Source software or hardware handshaking select"]
pub type HsSelSrcR = crate::BitReader<Handshaking>;
impl HsSelSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Handshaking {
        match self.bits {
            false => Handshaking::Hardware,
            true => Handshaking::Software,
        }
    }
    #[doc = "Hardware handshaking is used"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == Handshaking::Hardware
    }
    #[doc = "Software handshaking is used"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Handshaking::Software
    }
}
#[doc = "Field `hs_sel_src` writer - Source software or hardware handshaking select"]
pub type HsSelSrcW<'a, REG> = crate::BitWriter<'a, REG, Handshaking>;
impl<'a, REG> HsSelSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware handshaking is used"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(Handshaking::Hardware)
    }
    #[doc = "Software handshaking is used"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(Handshaking::Software)
    }
}
#[doc = "Field `hs_sel_dst` reader - Destination software or hardware handshaking select"]
pub use HsSelSrcR as HsSelDstR;
#[doc = "Field `hs_sel_dst` writer - Destination software or hardware handshaking select"]
pub use HsSelSrcW as HsSelDstW;
#[doc = "Source hardware handshaking interface polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    #[doc = "0: Active high"]
    ActiveHigh = 0,
    #[doc = "1: Active low"]
    ActiveLow = 1,
}
impl From<Polarity> for bool {
    #[inline(always)]
    fn from(variant: Polarity) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `src_hwhs_pol` reader - Source hardware handshaking interface polarity"]
pub type SrcHwhsPolR = crate::BitReader<Polarity>;
impl SrcHwhsPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity {
        match self.bits {
            false => Polarity::ActiveHigh,
            true => Polarity::ActiveLow,
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == Polarity::ActiveHigh
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == Polarity::ActiveLow
    }
}
#[doc = "Field `src_hwhs_pol` writer - Source hardware handshaking interface polarity"]
pub type SrcHwhsPolW<'a, REG> = crate::BitWriter<'a, REG, Polarity>;
impl<'a, REG> SrcHwhsPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::ActiveHigh)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::ActiveLow)
    }
}
#[doc = "Field `dst_hwhs_pol` reader - Destination hardware handshaking interface polarity"]
pub use SrcHwhsPolR as DstHwhsPolR;
#[doc = "Field `dst_hwhs_pol` writer - Destination hardware handshaking interface polarity"]
pub use SrcHwhsPolW as DstHwhsPolW;
#[doc = "Field `src_per` reader - Assign a hardware handshaking interface to source of channel"]
pub type SrcPerR = crate::FieldReader;
#[doc = "Field `src_per` writer - Assign a hardware handshaking interface to source of channel"]
pub type SrcPerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dst_per` reader - Assign a hardware handshaking interface to destination of channel"]
pub type DstPerR = crate::FieldReader;
#[doc = "Field `dst_per` writer - Assign a hardware handshaking interface to destination of channel"]
pub type DstPerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ch_prior` reader - Channel priority (7 is highest, 0 is lowest)"]
pub type ChPriorR = crate::FieldReader;
#[doc = "Field `ch_prior` writer - Channel priority (7 is highest, 0 is lowest)"]
pub type ChPriorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `lock_ch` reader - Channel lock bit"]
pub type LockChR = crate::BitReader;
#[doc = "Field `lock_ch` writer - Channel lock bit"]
pub type LockChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel lock level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockChL {
    #[doc = "0: Duration of channel is locked for entire DMA transfer"]
    DmaTransfer = 0,
    #[doc = "1: Duration of channel is locked for current block transfer"]
    BlockTransfer = 1,
    #[doc = "2: Duration of channel is locked for current transaction"]
    Transaction = 2,
}
impl From<LockChL> for u8 {
    #[inline(always)]
    fn from(variant: LockChL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockChL {
    type Ux = u8;
}
impl crate::IsEnum for LockChL {}
#[doc = "Field `lock_ch_l` reader - Channel lock level"]
pub type LockChLR = crate::FieldReader<LockChL>;
impl LockChLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockChL> {
        match self.bits {
            0 => Some(LockChL::DmaTransfer),
            1 => Some(LockChL::BlockTransfer),
            2 => Some(LockChL::Transaction),
            _ => None,
        }
    }
    #[doc = "Duration of channel is locked for entire DMA transfer"]
    #[inline(always)]
    pub fn is_dma_transfer(&self) -> bool {
        *self == LockChL::DmaTransfer
    }
    #[doc = "Duration of channel is locked for current block transfer"]
    #[inline(always)]
    pub fn is_block_transfer(&self) -> bool {
        *self == LockChL::BlockTransfer
    }
    #[doc = "Duration of channel is locked for current transaction"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        *self == LockChL::Transaction
    }
}
#[doc = "Field `lock_ch_l` writer - Channel lock level"]
pub type LockChLW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockChL>;
impl<'a, REG> LockChLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Duration of channel is locked for entire DMA transfer"]
    #[inline(always)]
    pub fn dma_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(LockChL::DmaTransfer)
    }
    #[doc = "Duration of channel is locked for current block transfer"]
    #[inline(always)]
    pub fn block_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(LockChL::BlockTransfer)
    }
    #[doc = "Duration of channel is locked for current transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut crate::W<REG> {
        self.variant(LockChL::Transaction)
    }
}
#[doc = "Field `src_osr_lmt` reader - Source outstanding request limit"]
pub type SrcOsrLmtR = crate::FieldReader;
#[doc = "Field `src_osr_lmt` writer - Source outstanding request limit"]
pub type SrcOsrLmtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dst_osr_lmt` reader - Destination outstanding request limit"]
pub type DstOsrLmtR = crate::FieldReader;
#[doc = "Field `dst_osr_lmt` writer - Destination outstanding request limit"]
pub type DstOsrLmtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Source multi-block transfer type"]
    #[inline(always)]
    pub fn src_multblk_type(&self) -> SrcMultblkTypeR {
        SrcMultblkTypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Destination multi-block transfer type"]
    #[inline(always)]
    pub fn dst_multblk_type(&self) -> DstMultblkTypeR {
        DstMultblkTypeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 32:34 - Transfer type and flow control"]
    #[inline(always)]
    pub fn tt_fc(&self) -> TtFcR {
        TtFcR::new(((self.bits >> 32) & 7) as u8)
    }
    #[doc = "Bit 35 - Source software or hardware handshaking select"]
    #[inline(always)]
    pub fn hs_sel_src(&self) -> HsSelSrcR {
        HsSelSrcR::new(((self.bits >> 35) & 1) != 0)
    }
    #[doc = "Bit 36 - Destination software or hardware handshaking select"]
    #[inline(always)]
    pub fn hs_sel_dst(&self) -> HsSelDstR {
        HsSelDstR::new(((self.bits >> 36) & 1) != 0)
    }
    #[doc = "Bit 37 - Source hardware handshaking interface polarity"]
    #[inline(always)]
    pub fn src_hwhs_pol(&self) -> SrcHwhsPolR {
        SrcHwhsPolR::new(((self.bits >> 37) & 1) != 0)
    }
    #[doc = "Bit 38 - Destination hardware handshaking interface polarity"]
    #[inline(always)]
    pub fn dst_hwhs_pol(&self) -> DstHwhsPolR {
        DstHwhsPolR::new(((self.bits >> 38) & 1) != 0)
    }
    #[doc = "Bits 39:42 - Assign a hardware handshaking interface to source of channel"]
    #[inline(always)]
    pub fn src_per(&self) -> SrcPerR {
        SrcPerR::new(((self.bits >> 39) & 0x0f) as u8)
    }
    #[doc = "Bits 44:47 - Assign a hardware handshaking interface to destination of channel"]
    #[inline(always)]
    pub fn dst_per(&self) -> DstPerR {
        DstPerR::new(((self.bits >> 44) & 0x0f) as u8)
    }
    #[doc = "Bits 49:51 - Channel priority (7 is highest, 0 is lowest)"]
    #[inline(always)]
    pub fn ch_prior(&self) -> ChPriorR {
        ChPriorR::new(((self.bits >> 49) & 7) as u8)
    }
    #[doc = "Bit 52 - Channel lock bit"]
    #[inline(always)]
    pub fn lock_ch(&self) -> LockChR {
        LockChR::new(((self.bits >> 52) & 1) != 0)
    }
    #[doc = "Bits 53:54 - Channel lock level"]
    #[inline(always)]
    pub fn lock_ch_l(&self) -> LockChLR {
        LockChLR::new(((self.bits >> 53) & 3) as u8)
    }
    #[doc = "Bits 55:58 - Source outstanding request limit"]
    #[inline(always)]
    pub fn src_osr_lmt(&self) -> SrcOsrLmtR {
        SrcOsrLmtR::new(((self.bits >> 55) & 0x0f) as u8)
    }
    #[doc = "Bits 59:62 - Destination outstanding request limit"]
    #[inline(always)]
    pub fn dst_osr_lmt(&self) -> DstOsrLmtR {
        DstOsrLmtR::new(((self.bits >> 59) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Source multi-block transfer type"]
    #[inline(always)]
    #[must_use]
    pub fn src_multblk_type(&mut self) -> SrcMultblkTypeW<CfgSpec> {
        SrcMultblkTypeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Destination multi-block transfer type"]
    #[inline(always)]
    #[must_use]
    pub fn dst_multblk_type(&mut self) -> DstMultblkTypeW<CfgSpec> {
        DstMultblkTypeW::new(self, 2)
    }
    #[doc = "Bits 32:34 - Transfer type and flow control"]
    #[inline(always)]
    #[must_use]
    pub fn tt_fc(&mut self) -> TtFcW<CfgSpec> {
        TtFcW::new(self, 32)
    }
    #[doc = "Bit 35 - Source software or hardware handshaking select"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel_src(&mut self) -> HsSelSrcW<CfgSpec> {
        HsSelSrcW::new(self, 35)
    }
    #[doc = "Bit 36 - Destination software or hardware handshaking select"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel_dst(&mut self) -> HsSelDstW<CfgSpec> {
        HsSelDstW::new(self, 36)
    }
    #[doc = "Bit 37 - Source hardware handshaking interface polarity"]
    #[inline(always)]
    #[must_use]
    pub fn src_hwhs_pol(&mut self) -> SrcHwhsPolW<CfgSpec> {
        SrcHwhsPolW::new(self, 37)
    }
    #[doc = "Bit 38 - Destination hardware handshaking interface polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dst_hwhs_pol(&mut self) -> DstHwhsPolW<CfgSpec> {
        DstHwhsPolW::new(self, 38)
    }
    #[doc = "Bits 39:42 - Assign a hardware handshaking interface to source of channel"]
    #[inline(always)]
    #[must_use]
    pub fn src_per(&mut self) -> SrcPerW<CfgSpec> {
        SrcPerW::new(self, 39)
    }
    #[doc = "Bits 44:47 - Assign a hardware handshaking interface to destination of channel"]
    #[inline(always)]
    #[must_use]
    pub fn dst_per(&mut self) -> DstPerW<CfgSpec> {
        DstPerW::new(self, 44)
    }
    #[doc = "Bits 49:51 - Channel priority (7 is highest, 0 is lowest)"]
    #[inline(always)]
    #[must_use]
    pub fn ch_prior(&mut self) -> ChPriorW<CfgSpec> {
        ChPriorW::new(self, 49)
    }
    #[doc = "Bit 52 - Channel lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ch(&mut self) -> LockChW<CfgSpec> {
        LockChW::new(self, 52)
    }
    #[doc = "Bits 53:54 - Channel lock level"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ch_l(&mut self) -> LockChLW<CfgSpec> {
        LockChLW::new(self, 53)
    }
    #[doc = "Bits 55:58 - Source outstanding request limit"]
    #[inline(always)]
    #[must_use]
    pub fn src_osr_lmt(&mut self) -> SrcOsrLmtW<CfgSpec> {
        SrcOsrLmtW::new(self, 55)
    }
    #[doc = "Bits 59:62 - Destination outstanding request limit"]
    #[inline(always)]
    #[must_use]
    pub fn dst_osr_lmt(&mut self) -> DstOsrLmtW<CfgSpec> {
        DstOsrLmtW::new(self, 59)
    }
}
#[doc = "Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets cfg to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u64 = 0;
}
