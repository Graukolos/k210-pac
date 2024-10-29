#[doc = "Register `con` reader"]
pub type R = crate::R<ConSpec>;
#[doc = "Register `con` writer"]
pub type W = crate::W<ConSpec>;
#[doc = "Field `master_mode` reader - Master Mode"]
pub type MasterModeR = crate::BitReader;
#[doc = "Field `master_mode` writer - Master Mode"]
pub type MasterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Speed {
    #[doc = "0: STANDARD"]
    Standard = 0,
    #[doc = "1: FAST"]
    Fast = 1,
    #[doc = "2: HIGHSPEED"]
    Highspeed = 2,
}
impl From<Speed> for u8 {
    #[inline(always)]
    fn from(variant: Speed) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Speed {
    type Ux = u8;
}
impl crate::IsEnum for Speed {}
#[doc = "Field `speed` reader - Speed"]
pub type SpeedR = crate::FieldReader<Speed>;
impl SpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Speed> {
        match self.bits {
            0 => Some(Speed::Standard),
            1 => Some(Speed::Fast),
            2 => Some(Speed::Highspeed),
            _ => None,
        }
    }
    #[doc = "STANDARD"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Speed::Standard
    }
    #[doc = "FAST"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Speed::Fast
    }
    #[doc = "HIGHSPEED"]
    #[inline(always)]
    pub fn is_highspeed(&self) -> bool {
        *self == Speed::Highspeed
    }
}
#[doc = "Field `speed` writer - Speed"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2, Speed>;
impl<'a, REG> SpeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "STANDARD"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Speed::Standard)
    }
    #[doc = "FAST"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Speed::Fast)
    }
    #[doc = "HIGHSPEED"]
    #[inline(always)]
    pub fn highspeed(self) -> &'a mut crate::W<REG> {
        self.variant(Speed::Highspeed)
    }
}
#[doc = "Slave address width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrSlaveWidth {
    #[doc = "0: 7-bit address"]
    B7 = 0,
    #[doc = "1: 10-bit address"]
    B10 = 1,
}
impl From<AddrSlaveWidth> for bool {
    #[inline(always)]
    fn from(variant: AddrSlaveWidth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `addr_slave_width` reader - Slave address width"]
pub type AddrSlaveWidthR = crate::BitReader<AddrSlaveWidth>;
impl AddrSlaveWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddrSlaveWidth {
        match self.bits {
            false => AddrSlaveWidth::B7,
            true => AddrSlaveWidth::B10,
        }
    }
    #[doc = "7-bit address"]
    #[inline(always)]
    pub fn is_b7(&self) -> bool {
        *self == AddrSlaveWidth::B7
    }
    #[doc = "10-bit address"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AddrSlaveWidth::B10
    }
}
#[doc = "Field `addr_slave_width` writer - Slave address width"]
pub type AddrSlaveWidthW<'a, REG> = crate::BitWriter<'a, REG, AddrSlaveWidth>;
impl<'a, REG> AddrSlaveWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit address"]
    #[inline(always)]
    pub fn b7(self) -> &'a mut crate::W<REG> {
        self.variant(AddrSlaveWidth::B7)
    }
    #[doc = "10-bit address"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AddrSlaveWidth::B10)
    }
}
#[doc = "Field `restart_en` reader - Enable Restart"]
pub type RestartEnR = crate::BitReader;
#[doc = "Field `restart_en` writer - Enable Restart"]
pub type RestartEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slave_disable` reader - Disable Slave"]
pub type SlaveDisableR = crate::BitReader;
#[doc = "Field `slave_disable` writer - Disable Slave"]
pub type SlaveDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stop_det` reader - STOP_DET_IFADDRESSED"]
pub type StopDetR = crate::BitReader;
#[doc = "Field `stop_det` writer - STOP_DET_IFADDRESSED"]
pub type StopDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_empty` reader - TX_EMPTY_CTRL"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `tx_empty` writer - TX_EMPTY_CTRL"]
pub type TxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Master Mode"]
    #[inline(always)]
    pub fn master_mode(&self) -> MasterModeR {
        MasterModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Slave address width"]
    #[inline(always)]
    pub fn addr_slave_width(&self) -> AddrSlaveWidthR {
        AddrSlaveWidthR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Restart"]
    #[inline(always)]
    pub fn restart_en(&self) -> RestartEnR {
        RestartEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Slave"]
    #[inline(always)]
    pub fn slave_disable(&self) -> SlaveDisableR {
        SlaveDisableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - STOP_DET_IFADDRESSED"]
    #[inline(always)]
    pub fn stop_det(&self) -> StopDetR {
        StopDetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TX_EMPTY_CTRL"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Mode"]
    #[inline(always)]
    #[must_use]
    pub fn master_mode(&mut self) -> MasterModeW<ConSpec> {
        MasterModeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Speed"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SpeedW<ConSpec> {
        SpeedW::new(self, 1)
    }
    #[doc = "Bit 3 - Slave address width"]
    #[inline(always)]
    #[must_use]
    pub fn addr_slave_width(&mut self) -> AddrSlaveWidthW<ConSpec> {
        AddrSlaveWidthW::new(self, 3)
    }
    #[doc = "Bit 5 - Enable Restart"]
    #[inline(always)]
    #[must_use]
    pub fn restart_en(&mut self) -> RestartEnW<ConSpec> {
        RestartEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Slave"]
    #[inline(always)]
    #[must_use]
    pub fn slave_disable(&mut self) -> SlaveDisableW<ConSpec> {
        SlaveDisableW::new(self, 6)
    }
    #[doc = "Bit 7 - STOP_DET_IFADDRESSED"]
    #[inline(always)]
    #[must_use]
    pub fn stop_det(&mut self) -> StopDetW<ConSpec> {
        StopDetW::new(self, 7)
    }
    #[doc = "Bit 8 - TX_EMPTY_CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TxEmptyW<ConSpec> {
        TxEmptyW::new(self, 8)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConSpec;
impl crate::RegisterSpec for ConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`con::R`](R) reader structure"]
impl crate::Readable for ConSpec {}
#[doc = "`write(|w| ..)` method takes [`con::W`](W) writer structure"]
impl crate::Writable for ConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets con to value 0"]
impl crate::Resettable for ConSpec {
    const RESET_VALUE: u32 = 0;
}
