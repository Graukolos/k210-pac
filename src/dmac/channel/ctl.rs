#[doc = "Register `ctl` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `ctl` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Source master select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MasterSelect {
    #[doc = "0: AXI master 1"]
    AxiMaster1 = 0,
    #[doc = "1: AXI master 2"]
    AxiMaster2 = 1,
}
impl From<MasterSelect> for bool {
    #[inline(always)]
    fn from(variant: MasterSelect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sms` reader - Source master select"]
pub type SmsR = crate::BitReader<MasterSelect>;
impl SmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MasterSelect {
        match self.bits {
            false => MasterSelect::AxiMaster1,
            true => MasterSelect::AxiMaster2,
        }
    }
    #[doc = "AXI master 1"]
    #[inline(always)]
    pub fn is_axi_master_1(&self) -> bool {
        *self == MasterSelect::AxiMaster1
    }
    #[doc = "AXI master 2"]
    #[inline(always)]
    pub fn is_axi_master_2(&self) -> bool {
        *self == MasterSelect::AxiMaster2
    }
}
#[doc = "Field `sms` writer - Source master select"]
pub type SmsW<'a, REG> = crate::BitWriter<'a, REG, MasterSelect>;
impl<'a, REG> SmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AXI master 1"]
    #[inline(always)]
    pub fn axi_master_1(self) -> &'a mut crate::W<REG> {
        self.variant(MasterSelect::AxiMaster1)
    }
    #[doc = "AXI master 2"]
    #[inline(always)]
    pub fn axi_master_2(self) -> &'a mut crate::W<REG> {
        self.variant(MasterSelect::AxiMaster2)
    }
}
#[doc = "Field `dms` reader - Destination master select"]
pub use SmsR as DmsR;
#[doc = "Field `dms` writer - Destination master select"]
pub use SmsW as DmsW;
#[doc = "Source address increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Increment {
    #[doc = "0: Increment address"]
    Increment = 0,
    #[doc = "1: Don't increment address"]
    Nochange = 1,
}
impl From<Increment> for bool {
    #[inline(always)]
    fn from(variant: Increment) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sinc` reader - Source address increment"]
pub type SincR = crate::BitReader<Increment>;
impl SincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Increment {
        match self.bits {
            false => Increment::Increment,
            true => Increment::Nochange,
        }
    }
    #[doc = "Increment address"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == Increment::Increment
    }
    #[doc = "Don't increment address"]
    #[inline(always)]
    pub fn is_nochange(&self) -> bool {
        *self == Increment::Nochange
    }
}
#[doc = "Field `sinc` writer - Source address increment"]
pub type SincW<'a, REG> = crate::BitWriter<'a, REG, Increment>;
impl<'a, REG> SincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Increment address"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(Increment::Increment)
    }
    #[doc = "Don't increment address"]
    #[inline(always)]
    pub fn nochange(self) -> &'a mut crate::W<REG> {
        self.variant(Increment::Nochange)
    }
}
#[doc = "Field `dinc` reader - Destination address increment"]
pub use SincR as DincR;
#[doc = "Field `dinc` writer - Destination address increment"]
pub use SincW as DincW;
#[doc = "Source transfer width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TransferWidth {
    #[doc = "0: 8 bits"]
    Width8 = 0,
    #[doc = "1: 16 bits"]
    Width16 = 1,
    #[doc = "2: 32 bits"]
    Width32 = 2,
    #[doc = "3: 64 bits"]
    Width64 = 3,
    #[doc = "4: 128 bits"]
    Width128 = 4,
    #[doc = "5: 256 bits"]
    Width256 = 5,
    #[doc = "6: 512 bits"]
    Width512 = 6,
}
impl From<TransferWidth> for u8 {
    #[inline(always)]
    fn from(variant: TransferWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TransferWidth {
    type Ux = u8;
}
impl crate::IsEnum for TransferWidth {}
#[doc = "Field `src_tr_width` reader - Source transfer width"]
pub type SrcTrWidthR = crate::FieldReader<TransferWidth>;
impl SrcTrWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TransferWidth> {
        match self.bits {
            0 => Some(TransferWidth::Width8),
            1 => Some(TransferWidth::Width16),
            2 => Some(TransferWidth::Width32),
            3 => Some(TransferWidth::Width64),
            4 => Some(TransferWidth::Width128),
            5 => Some(TransferWidth::Width256),
            6 => Some(TransferWidth::Width512),
            _ => None,
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_width_8(&self) -> bool {
        *self == TransferWidth::Width8
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_width_16(&self) -> bool {
        *self == TransferWidth::Width16
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_width_32(&self) -> bool {
        *self == TransferWidth::Width32
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn is_width_64(&self) -> bool {
        *self == TransferWidth::Width64
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn is_width_128(&self) -> bool {
        *self == TransferWidth::Width128
    }
    #[doc = "256 bits"]
    #[inline(always)]
    pub fn is_width_256(&self) -> bool {
        *self == TransferWidth::Width256
    }
    #[doc = "512 bits"]
    #[inline(always)]
    pub fn is_width_512(&self) -> bool {
        *self == TransferWidth::Width512
    }
}
#[doc = "Field `src_tr_width` writer - Source transfer width"]
pub type SrcTrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3, TransferWidth>;
impl<'a, REG> SrcTrWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn width_8(self) -> &'a mut crate::W<REG> {
        self.variant(TransferWidth::Width8)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn width_16(self) -> &'a mut crate::W<REG> {
        self.variant(TransferWidth::Width16)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn width_32(self) -> &'a mut crate::W<REG> {
        self.variant(TransferWidth::Width32)
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn width_64(self) -> &'a mut crate::W<REG> {
        self.variant(TransferWidth::Width64)
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn width_128(self) -> &'a mut crate::W<REG> {
        self.variant(TransferWidth::Width128)
    }
    #[doc = "256 bits"]
    #[inline(always)]
    pub fn width_256(self) -> &'a mut crate::W<REG> {
        self.variant(TransferWidth::Width256)
    }
    #[doc = "512 bits"]
    #[inline(always)]
    pub fn width_512(self) -> &'a mut crate::W<REG> {
        self.variant(TransferWidth::Width512)
    }
}
#[doc = "Field `dst_tr_width` reader - Destination transfer width"]
pub use SrcTrWidthR as DstTrWidthR;
#[doc = "Field `dst_tr_width` writer - Destination transfer width"]
pub use SrcTrWidthW as DstTrWidthW;
#[doc = "Source burst transaction length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BurstLength {
    #[doc = "0: 1 data item"]
    Length1 = 0,
    #[doc = "1: 4 data items"]
    Length4 = 1,
    #[doc = "2: 8 data items"]
    Length8 = 2,
    #[doc = "3: 16 data items"]
    Length16 = 3,
    #[doc = "4: 32 data items"]
    Length32 = 4,
    #[doc = "5: 64 data items"]
    Length64 = 5,
    #[doc = "6: 128 data items"]
    Length128 = 6,
    #[doc = "7: 256 data items"]
    Length256 = 7,
    #[doc = "8: 512 data items"]
    Length512 = 8,
    #[doc = "9: 1024 data items"]
    Length1024 = 9,
}
impl From<BurstLength> for u8 {
    #[inline(always)]
    fn from(variant: BurstLength) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BurstLength {
    type Ux = u8;
}
impl crate::IsEnum for BurstLength {}
#[doc = "Field `src_msize` reader - Source burst transaction length"]
pub type SrcMsizeR = crate::FieldReader<BurstLength>;
impl SrcMsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BurstLength> {
        match self.bits {
            0 => Some(BurstLength::Length1),
            1 => Some(BurstLength::Length4),
            2 => Some(BurstLength::Length8),
            3 => Some(BurstLength::Length16),
            4 => Some(BurstLength::Length32),
            5 => Some(BurstLength::Length64),
            6 => Some(BurstLength::Length128),
            7 => Some(BurstLength::Length256),
            8 => Some(BurstLength::Length512),
            9 => Some(BurstLength::Length1024),
            _ => None,
        }
    }
    #[doc = "1 data item"]
    #[inline(always)]
    pub fn is_length_1(&self) -> bool {
        *self == BurstLength::Length1
    }
    #[doc = "4 data items"]
    #[inline(always)]
    pub fn is_length_4(&self) -> bool {
        *self == BurstLength::Length4
    }
    #[doc = "8 data items"]
    #[inline(always)]
    pub fn is_length_8(&self) -> bool {
        *self == BurstLength::Length8
    }
    #[doc = "16 data items"]
    #[inline(always)]
    pub fn is_length_16(&self) -> bool {
        *self == BurstLength::Length16
    }
    #[doc = "32 data items"]
    #[inline(always)]
    pub fn is_length_32(&self) -> bool {
        *self == BurstLength::Length32
    }
    #[doc = "64 data items"]
    #[inline(always)]
    pub fn is_length_64(&self) -> bool {
        *self == BurstLength::Length64
    }
    #[doc = "128 data items"]
    #[inline(always)]
    pub fn is_length_128(&self) -> bool {
        *self == BurstLength::Length128
    }
    #[doc = "256 data items"]
    #[inline(always)]
    pub fn is_length_256(&self) -> bool {
        *self == BurstLength::Length256
    }
    #[doc = "512 data items"]
    #[inline(always)]
    pub fn is_length_512(&self) -> bool {
        *self == BurstLength::Length512
    }
    #[doc = "1024 data items"]
    #[inline(always)]
    pub fn is_length_1024(&self) -> bool {
        *self == BurstLength::Length1024
    }
}
#[doc = "Field `src_msize` writer - Source burst transaction length"]
pub type SrcMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4, BurstLength>;
impl<'a, REG> SrcMsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 data item"]
    #[inline(always)]
    pub fn length_1(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLength::Length1)
    }
    #[doc = "4 data items"]
    #[inline(always)]
    pub fn length_4(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLength::Length4)
    }
    #[doc = "8 data items"]
    #[inline(always)]
    pub fn length_8(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLength::Length8)
    }
    #[doc = "16 data items"]
    #[inline(always)]
    pub fn length_16(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLength::Length16)
    }
    #[doc = "32 data items"]
    #[inline(always)]
    pub fn length_32(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLength::Length32)
    }
    #[doc = "64 data items"]
    #[inline(always)]
    pub fn length_64(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLength::Length64)
    }
    #[doc = "128 data items"]
    #[inline(always)]
    pub fn length_128(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLength::Length128)
    }
    #[doc = "256 data items"]
    #[inline(always)]
    pub fn length_256(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLength::Length256)
    }
    #[doc = "512 data items"]
    #[inline(always)]
    pub fn length_512(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLength::Length512)
    }
    #[doc = "1024 data items"]
    #[inline(always)]
    pub fn length_1024(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLength::Length1024)
    }
}
#[doc = "Field `dst_msize` reader - Destination burst transaction length"]
pub use SrcMsizeR as DstMsizeR;
#[doc = "Field `dst_msize` writer - Destination burst transaction length"]
pub use SrcMsizeW as DstMsizeW;
#[doc = "Field `nonposted_lastwrite_en` reader - Non Posted Last Write Enable (posted writes may be used till the end of the block)"]
pub type NonpostedLastwriteEnR = crate::BitReader;
#[doc = "Field `nonposted_lastwrite_en` writer - Non Posted Last Write Enable (posted writes may be used till the end of the block)"]
pub type NonpostedLastwriteEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `arlen_en` reader - Source burst length enable"]
pub type ArlenEnR = crate::BitReader;
#[doc = "Field `arlen_en` writer - Source burst length enable"]
pub type ArlenEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `arlen` reader - Source burst length"]
pub type ArlenR = crate::FieldReader;
#[doc = "Field `arlen` writer - Source burst length"]
pub type ArlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `awlen_en` reader - Destination burst length enable"]
pub type AwlenEnR = crate::BitReader;
#[doc = "Field `awlen_en` writer - Destination burst length enable"]
pub type AwlenEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `awlen` reader - Destination burst length"]
pub type AwlenR = crate::FieldReader;
#[doc = "Field `awlen` writer - Destination burst length"]
pub type AwlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `src_stat_en` reader - Source status enable"]
pub type SrcStatEnR = crate::BitReader;
#[doc = "Field `src_stat_en` writer - Source status enable"]
pub type SrcStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dst_stat_en` reader - Destination status enable"]
pub type DstStatEnR = crate::BitReader;
#[doc = "Field `dst_stat_en` writer - Destination status enable"]
pub type DstStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ioc_blktfr` reader - Interrupt completion of block transfer"]
pub type IocBlktfrR = crate::BitReader;
#[doc = "Field `ioc_blktfr` writer - Interrupt completion of block transfer"]
pub type IocBlktfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `shadowreg_or_lli_last` reader - Last shadow linked list item (indicates shadowreg/LLI content is the last one)"]
pub type ShadowregOrLliLastR = crate::BitReader;
#[doc = "Field `shadowreg_or_lli_last` writer - Last shadow linked list item (indicates shadowreg/LLI content is the last one)"]
pub type ShadowregOrLliLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `shadowreg_or_lli_valid` reader - last shadow linked list item valid (indicate shadowreg/LLI content is valid)"]
pub type ShadowregOrLliValidR = crate::BitReader;
#[doc = "Field `shadowreg_or_lli_valid` writer - last shadow linked list item valid (indicate shadowreg/LLI content is valid)"]
pub type ShadowregOrLliValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Source master select"]
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Destination master select"]
    #[inline(always)]
    pub fn dms(&self) -> DmsR {
        DmsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Source address increment"]
    #[inline(always)]
    pub fn sinc(&self) -> SincR {
        SincR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Destination address increment"]
    #[inline(always)]
    pub fn dinc(&self) -> DincR {
        DincR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Source transfer width"]
    #[inline(always)]
    pub fn src_tr_width(&self) -> SrcTrWidthR {
        SrcTrWidthR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Destination transfer width"]
    #[inline(always)]
    pub fn dst_tr_width(&self) -> DstTrWidthR {
        DstTrWidthR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:17 - Source burst transaction length"]
    #[inline(always)]
    pub fn src_msize(&self) -> SrcMsizeR {
        SrcMsizeR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - Destination burst transaction length"]
    #[inline(always)]
    pub fn dst_msize(&self) -> DstMsizeR {
        DstMsizeR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Non Posted Last Write Enable (posted writes may be used till the end of the block)"]
    #[inline(always)]
    pub fn nonposted_lastwrite_en(&self) -> NonpostedLastwriteEnR {
        NonpostedLastwriteEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 38 - Source burst length enable"]
    #[inline(always)]
    pub fn arlen_en(&self) -> ArlenEnR {
        ArlenEnR::new(((self.bits >> 38) & 1) != 0)
    }
    #[doc = "Bits 39:46 - Source burst length"]
    #[inline(always)]
    pub fn arlen(&self) -> ArlenR {
        ArlenR::new(((self.bits >> 39) & 0xff) as u8)
    }
    #[doc = "Bit 47 - Destination burst length enable"]
    #[inline(always)]
    pub fn awlen_en(&self) -> AwlenEnR {
        AwlenEnR::new(((self.bits >> 47) & 1) != 0)
    }
    #[doc = "Bits 48:55 - Destination burst length"]
    #[inline(always)]
    pub fn awlen(&self) -> AwlenR {
        AwlenR::new(((self.bits >> 48) & 0xff) as u8)
    }
    #[doc = "Bit 56 - Source status enable"]
    #[inline(always)]
    pub fn src_stat_en(&self) -> SrcStatEnR {
        SrcStatEnR::new(((self.bits >> 56) & 1) != 0)
    }
    #[doc = "Bit 57 - Destination status enable"]
    #[inline(always)]
    pub fn dst_stat_en(&self) -> DstStatEnR {
        DstStatEnR::new(((self.bits >> 57) & 1) != 0)
    }
    #[doc = "Bit 58 - Interrupt completion of block transfer"]
    #[inline(always)]
    pub fn ioc_blktfr(&self) -> IocBlktfrR {
        IocBlktfrR::new(((self.bits >> 58) & 1) != 0)
    }
    #[doc = "Bit 62 - Last shadow linked list item (indicates shadowreg/LLI content is the last one)"]
    #[inline(always)]
    pub fn shadowreg_or_lli_last(&self) -> ShadowregOrLliLastR {
        ShadowregOrLliLastR::new(((self.bits >> 62) & 1) != 0)
    }
    #[doc = "Bit 63 - last shadow linked list item valid (indicate shadowreg/LLI content is valid)"]
    #[inline(always)]
    pub fn shadowreg_or_lli_valid(&self) -> ShadowregOrLliValidR {
        ShadowregOrLliValidR::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source master select"]
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SmsW<CtlSpec> {
        SmsW::new(self, 0)
    }
    #[doc = "Bit 2 - Destination master select"]
    #[inline(always)]
    #[must_use]
    pub fn dms(&mut self) -> DmsW<CtlSpec> {
        DmsW::new(self, 2)
    }
    #[doc = "Bit 4 - Source address increment"]
    #[inline(always)]
    #[must_use]
    pub fn sinc(&mut self) -> SincW<CtlSpec> {
        SincW::new(self, 4)
    }
    #[doc = "Bit 6 - Destination address increment"]
    #[inline(always)]
    #[must_use]
    pub fn dinc(&mut self) -> DincW<CtlSpec> {
        DincW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Source transfer width"]
    #[inline(always)]
    #[must_use]
    pub fn src_tr_width(&mut self) -> SrcTrWidthW<CtlSpec> {
        SrcTrWidthW::new(self, 8)
    }
    #[doc = "Bits 11:13 - Destination transfer width"]
    #[inline(always)]
    #[must_use]
    pub fn dst_tr_width(&mut self) -> DstTrWidthW<CtlSpec> {
        DstTrWidthW::new(self, 11)
    }
    #[doc = "Bits 14:17 - Source burst transaction length"]
    #[inline(always)]
    #[must_use]
    pub fn src_msize(&mut self) -> SrcMsizeW<CtlSpec> {
        SrcMsizeW::new(self, 14)
    }
    #[doc = "Bits 18:21 - Destination burst transaction length"]
    #[inline(always)]
    #[must_use]
    pub fn dst_msize(&mut self) -> DstMsizeW<CtlSpec> {
        DstMsizeW::new(self, 18)
    }
    #[doc = "Bit 30 - Non Posted Last Write Enable (posted writes may be used till the end of the block)"]
    #[inline(always)]
    #[must_use]
    pub fn nonposted_lastwrite_en(&mut self) -> NonpostedLastwriteEnW<CtlSpec> {
        NonpostedLastwriteEnW::new(self, 30)
    }
    #[doc = "Bit 38 - Source burst length enable"]
    #[inline(always)]
    #[must_use]
    pub fn arlen_en(&mut self) -> ArlenEnW<CtlSpec> {
        ArlenEnW::new(self, 38)
    }
    #[doc = "Bits 39:46 - Source burst length"]
    #[inline(always)]
    #[must_use]
    pub fn arlen(&mut self) -> ArlenW<CtlSpec> {
        ArlenW::new(self, 39)
    }
    #[doc = "Bit 47 - Destination burst length enable"]
    #[inline(always)]
    #[must_use]
    pub fn awlen_en(&mut self) -> AwlenEnW<CtlSpec> {
        AwlenEnW::new(self, 47)
    }
    #[doc = "Bits 48:55 - Destination burst length"]
    #[inline(always)]
    #[must_use]
    pub fn awlen(&mut self) -> AwlenW<CtlSpec> {
        AwlenW::new(self, 48)
    }
    #[doc = "Bit 56 - Source status enable"]
    #[inline(always)]
    #[must_use]
    pub fn src_stat_en(&mut self) -> SrcStatEnW<CtlSpec> {
        SrcStatEnW::new(self, 56)
    }
    #[doc = "Bit 57 - Destination status enable"]
    #[inline(always)]
    #[must_use]
    pub fn dst_stat_en(&mut self) -> DstStatEnW<CtlSpec> {
        DstStatEnW::new(self, 57)
    }
    #[doc = "Bit 58 - Interrupt completion of block transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ioc_blktfr(&mut self) -> IocBlktfrW<CtlSpec> {
        IocBlktfrW::new(self, 58)
    }
    #[doc = "Bit 62 - Last shadow linked list item (indicates shadowreg/LLI content is the last one)"]
    #[inline(always)]
    #[must_use]
    pub fn shadowreg_or_lli_last(&mut self) -> ShadowregOrLliLastW<CtlSpec> {
        ShadowregOrLliLastW::new(self, 62)
    }
    #[doc = "Bit 63 - last shadow linked list item valid (indicate shadowreg/LLI content is valid)"]
    #[inline(always)]
    #[must_use]
    pub fn shadowreg_or_lli_valid(&mut self) -> ShadowregOrLliValidW<CtlSpec> {
        ShadowregOrLliValidW::new(self, 63)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets ctl to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u64 = 0;
}
