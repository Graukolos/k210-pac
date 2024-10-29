#[doc = "Register `ccr` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `ccr` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Gating of sclk\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkGate {
    #[doc = "0: Clock gating is disabled"]
    No = 0,
    #[doc = "1: Gating after 12 sclk cycles"]
    Cycles12 = 1,
    #[doc = "2: Gating after 16 sclk cycles"]
    Cycles16 = 2,
    #[doc = "3: Gating after 20 sclk cycles"]
    Cycles20 = 3,
    #[doc = "4: Gating after 24 sclk cycles"]
    Cycles24 = 4,
}
impl From<ClkGate> for u8 {
    #[inline(always)]
    fn from(variant: ClkGate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkGate {
    type Ux = u8;
}
impl crate::IsEnum for ClkGate {}
#[doc = "Field `clk_gate` reader - Gating of sclk"]
pub type ClkGateR = crate::FieldReader<ClkGate>;
impl ClkGateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkGate> {
        match self.bits {
            0 => Some(ClkGate::No),
            1 => Some(ClkGate::Cycles12),
            2 => Some(ClkGate::Cycles16),
            3 => Some(ClkGate::Cycles20),
            4 => Some(ClkGate::Cycles24),
            _ => None,
        }
    }
    #[doc = "Clock gating is disabled"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ClkGate::No
    }
    #[doc = "Gating after 12 sclk cycles"]
    #[inline(always)]
    pub fn is_cycles12(&self) -> bool {
        *self == ClkGate::Cycles12
    }
    #[doc = "Gating after 16 sclk cycles"]
    #[inline(always)]
    pub fn is_cycles16(&self) -> bool {
        *self == ClkGate::Cycles16
    }
    #[doc = "Gating after 20 sclk cycles"]
    #[inline(always)]
    pub fn is_cycles20(&self) -> bool {
        *self == ClkGate::Cycles20
    }
    #[doc = "Gating after 24 sclk cycles"]
    #[inline(always)]
    pub fn is_cycles24(&self) -> bool {
        *self == ClkGate::Cycles24
    }
}
#[doc = "Field `clk_gate` writer - Gating of sclk"]
pub type ClkGateW<'a, REG> = crate::FieldWriter<'a, REG, 3, ClkGate>;
impl<'a, REG> ClkGateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock gating is disabled"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(ClkGate::No)
    }
    #[doc = "Gating after 12 sclk cycles"]
    #[inline(always)]
    pub fn cycles12(self) -> &'a mut crate::W<REG> {
        self.variant(ClkGate::Cycles12)
    }
    #[doc = "Gating after 16 sclk cycles"]
    #[inline(always)]
    pub fn cycles16(self) -> &'a mut crate::W<REG> {
        self.variant(ClkGate::Cycles16)
    }
    #[doc = "Gating after 20 sclk cycles"]
    #[inline(always)]
    pub fn cycles20(self) -> &'a mut crate::W<REG> {
        self.variant(ClkGate::Cycles20)
    }
    #[doc = "Gating after 24 sclk cycles"]
    #[inline(always)]
    pub fn cycles24(self) -> &'a mut crate::W<REG> {
        self.variant(ClkGate::Cycles24)
    }
}
#[doc = "The number of sclk cycles for which the word select line stayd in the left aligned or right aligned mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkWordSize {
    #[doc = "0: 16 sclk cycles"]
    Cycles16 = 0,
    #[doc = "1: 24 sclk cycles"]
    Cycles24 = 1,
    #[doc = "2: 32 sclk cycles"]
    Cycles32 = 2,
}
impl From<ClkWordSize> for u8 {
    #[inline(always)]
    fn from(variant: ClkWordSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkWordSize {
    type Ux = u8;
}
impl crate::IsEnum for ClkWordSize {}
#[doc = "Field `clk_word_size` reader - The number of sclk cycles for which the word select line stayd in the left aligned or right aligned mode"]
pub type ClkWordSizeR = crate::FieldReader<ClkWordSize>;
impl ClkWordSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkWordSize> {
        match self.bits {
            0 => Some(ClkWordSize::Cycles16),
            1 => Some(ClkWordSize::Cycles24),
            2 => Some(ClkWordSize::Cycles32),
            _ => None,
        }
    }
    #[doc = "16 sclk cycles"]
    #[inline(always)]
    pub fn is_cycles16(&self) -> bool {
        *self == ClkWordSize::Cycles16
    }
    #[doc = "24 sclk cycles"]
    #[inline(always)]
    pub fn is_cycles24(&self) -> bool {
        *self == ClkWordSize::Cycles24
    }
    #[doc = "32 sclk cycles"]
    #[inline(always)]
    pub fn is_cycles32(&self) -> bool {
        *self == ClkWordSize::Cycles32
    }
}
#[doc = "Field `clk_word_size` writer - The number of sclk cycles for which the word select line stayd in the left aligned or right aligned mode"]
pub type ClkWordSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkWordSize>;
impl<'a, REG> ClkWordSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16 sclk cycles"]
    #[inline(always)]
    pub fn cycles16(self) -> &'a mut crate::W<REG> {
        self.variant(ClkWordSize::Cycles16)
    }
    #[doc = "24 sclk cycles"]
    #[inline(always)]
    pub fn cycles24(self) -> &'a mut crate::W<REG> {
        self.variant(ClkWordSize::Cycles24)
    }
    #[doc = "32 sclk cycles"]
    #[inline(always)]
    pub fn cycles32(self) -> &'a mut crate::W<REG> {
        self.variant(ClkWordSize::Cycles32)
    }
}
#[doc = "Alignment mode setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AlignMode {
    #[doc = "1: Standard I2S format"]
    Standard = 1,
    #[doc = "2: Right aligned format"]
    Right = 2,
    #[doc = "4: Left aligned format"]
    Left = 4,
}
impl From<AlignMode> for u8 {
    #[inline(always)]
    fn from(variant: AlignMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AlignMode {
    type Ux = u8;
}
impl crate::IsEnum for AlignMode {}
#[doc = "Field `align_mode` reader - Alignment mode setting"]
pub type AlignModeR = crate::FieldReader<AlignMode>;
impl AlignModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AlignMode> {
        match self.bits {
            1 => Some(AlignMode::Standard),
            2 => Some(AlignMode::Right),
            4 => Some(AlignMode::Left),
            _ => None,
        }
    }
    #[doc = "Standard I2S format"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == AlignMode::Standard
    }
    #[doc = "Right aligned format"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == AlignMode::Right
    }
    #[doc = "Left aligned format"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == AlignMode::Left
    }
}
#[doc = "Field `align_mode` writer - Alignment mode setting"]
pub type AlignModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, AlignMode>;
impl<'a, REG> AlignModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard I2S format"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(AlignMode::Standard)
    }
    #[doc = "Right aligned format"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(AlignMode::Right)
    }
    #[doc = "Left aligned format"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(AlignMode::Left)
    }
}
#[doc = "Field `dma_tx_en` reader - DMA transmit enable control"]
pub type DmaTxEnR = crate::BitReader;
#[doc = "Field `dma_tx_en` writer - DMA transmit enable control"]
pub type DmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_rx_en` reader - DMA receive enable control"]
pub type DmaRxEnR = crate::BitReader;
#[doc = "Field `dma_rx_en` writer - DMA receive enable control"]
pub type DmaRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_divide_16` reader - Split 32bit data to two 16 bit data and filled in left and right channel. Used with dma_tx_en or dma_rx_en"]
pub type DmaDivide16R = crate::BitReader;
#[doc = "Field `dma_divide_16` writer - Split 32bit data to two 16 bit data and filled in left and right channel. Used with dma_tx_en or dma_rx_en"]
pub type DmaDivide16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sign_expand_en` reader - SIGN_EXPAND_EN"]
pub type SignExpandEnR = crate::BitReader;
#[doc = "Field `sign_expand_en` writer - SIGN_EXPAND_EN"]
pub type SignExpandEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Gating of sclk"]
    #[inline(always)]
    pub fn clk_gate(&self) -> ClkGateR {
        ClkGateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - The number of sclk cycles for which the word select line stayd in the left aligned or right aligned mode"]
    #[inline(always)]
    pub fn clk_word_size(&self) -> ClkWordSizeR {
        ClkWordSizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Alignment mode setting"]
    #[inline(always)]
    pub fn align_mode(&self) -> AlignModeR {
        AlignModeR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - DMA transmit enable control"]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DmaTxEnR {
        DmaTxEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA receive enable control"]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DmaRxEnR {
        DmaRxEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Split 32bit data to two 16 bit data and filled in left and right channel. Used with dma_tx_en or dma_rx_en"]
    #[inline(always)]
    pub fn dma_divide_16(&self) -> DmaDivide16R {
        DmaDivide16R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SIGN_EXPAND_EN"]
    #[inline(always)]
    pub fn sign_expand_en(&self) -> SignExpandEnR {
        SignExpandEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Gating of sclk"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gate(&mut self) -> ClkGateW<CcrSpec> {
        ClkGateW::new(self, 0)
    }
    #[doc = "Bits 3:4 - The number of sclk cycles for which the word select line stayd in the left aligned or right aligned mode"]
    #[inline(always)]
    #[must_use]
    pub fn clk_word_size(&mut self) -> ClkWordSizeW<CcrSpec> {
        ClkWordSizeW::new(self, 3)
    }
    #[doc = "Bits 5:7 - Alignment mode setting"]
    #[inline(always)]
    #[must_use]
    pub fn align_mode(&mut self) -> AlignModeW<CcrSpec> {
        AlignModeW::new(self, 5)
    }
    #[doc = "Bit 8 - DMA transmit enable control"]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_en(&mut self) -> DmaTxEnW<CcrSpec> {
        DmaTxEnW::new(self, 8)
    }
    #[doc = "Bit 9 - DMA receive enable control"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_en(&mut self) -> DmaRxEnW<CcrSpec> {
        DmaRxEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Split 32bit data to two 16 bit data and filled in left and right channel. Used with dma_tx_en or dma_rx_en"]
    #[inline(always)]
    #[must_use]
    pub fn dma_divide_16(&mut self) -> DmaDivide16W<CcrSpec> {
        DmaDivide16W::new(self, 10)
    }
    #[doc = "Bit 11 - SIGN_EXPAND_EN"]
    #[inline(always)]
    #[must_use]
    pub fn sign_expand_en(&mut self) -> SignExpandEnW<CcrSpec> {
        SignExpandEnW::new(self, 11)
    }
}
#[doc = "Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ccr to value 0"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0;
}
