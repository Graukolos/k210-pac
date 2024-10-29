#[doc = "Register `spi_ctrlr0` reader"]
pub type R = crate::R<SpiCtrlr0Spec>;
#[doc = "Register `spi_ctrlr0` writer"]
pub type W = crate::W<SpiCtrlr0Spec>;
#[doc = "instruction_address_trans_mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aitm {
    #[doc = "0: STANDARD"]
    Standard = 0,
    #[doc = "1: ADDR_STANDARD"]
    AddrStandard = 1,
    #[doc = "2: AS_FRAME_FORMAT"]
    AsFrameFormat = 2,
}
impl From<Aitm> for u8 {
    #[inline(always)]
    fn from(variant: Aitm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aitm {
    type Ux = u8;
}
impl crate::IsEnum for Aitm {}
#[doc = "Field `aitm` reader - instruction_address_trans_mode"]
pub type AitmR = crate::FieldReader<Aitm>;
impl AitmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aitm> {
        match self.bits {
            0 => Some(Aitm::Standard),
            1 => Some(Aitm::AddrStandard),
            2 => Some(Aitm::AsFrameFormat),
            _ => None,
        }
    }
    #[doc = "STANDARD"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Aitm::Standard
    }
    #[doc = "ADDR_STANDARD"]
    #[inline(always)]
    pub fn is_addr_standard(&self) -> bool {
        *self == Aitm::AddrStandard
    }
    #[doc = "AS_FRAME_FORMAT"]
    #[inline(always)]
    pub fn is_as_frame_format(&self) -> bool {
        *self == Aitm::AsFrameFormat
    }
}
#[doc = "Field `aitm` writer - instruction_address_trans_mode"]
pub type AitmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aitm>;
impl<'a, REG> AitmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "STANDARD"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Aitm::Standard)
    }
    #[doc = "ADDR_STANDARD"]
    #[inline(always)]
    pub fn addr_standard(self) -> &'a mut crate::W<REG> {
        self.variant(Aitm::AddrStandard)
    }
    #[doc = "AS_FRAME_FORMAT"]
    #[inline(always)]
    pub fn as_frame_format(self) -> &'a mut crate::W<REG> {
        self.variant(Aitm::AsFrameFormat)
    }
}
#[doc = "Field `addr_length` reader - ADDR_LENGTH"]
pub type AddrLengthR = crate::FieldReader;
#[doc = "Field `addr_length` writer - ADDR_LENGTH"]
pub type AddrLengthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `inst_length` reader - INSTRUCTION_LENGTH"]
pub type InstLengthR = crate::FieldReader;
#[doc = "Field `inst_length` writer - INSTRUCTION_LENGTH"]
pub type InstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wait_cycles` reader - WAIT_CYCLES"]
pub type WaitCyclesR = crate::FieldReader;
#[doc = "Field `wait_cycles` writer - WAIT_CYCLES"]
pub type WaitCyclesW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:1 - instruction_address_trans_mode"]
    #[inline(always)]
    pub fn aitm(&self) -> AitmR {
        AitmR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - ADDR_LENGTH"]
    #[inline(always)]
    pub fn addr_length(&self) -> AddrLengthR {
        AddrLengthR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - INSTRUCTION_LENGTH"]
    #[inline(always)]
    pub fn inst_length(&self) -> InstLengthR {
        InstLengthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:15 - WAIT_CYCLES"]
    #[inline(always)]
    pub fn wait_cycles(&self) -> WaitCyclesR {
        WaitCyclesR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - instruction_address_trans_mode"]
    #[inline(always)]
    #[must_use]
    pub fn aitm(&mut self) -> AitmW<SpiCtrlr0Spec> {
        AitmW::new(self, 0)
    }
    #[doc = "Bits 2:5 - ADDR_LENGTH"]
    #[inline(always)]
    #[must_use]
    pub fn addr_length(&mut self) -> AddrLengthW<SpiCtrlr0Spec> {
        AddrLengthW::new(self, 2)
    }
    #[doc = "Bits 8:9 - INSTRUCTION_LENGTH"]
    #[inline(always)]
    #[must_use]
    pub fn inst_length(&mut self) -> InstLengthW<SpiCtrlr0Spec> {
        InstLengthW::new(self, 8)
    }
    #[doc = "Bits 11:15 - WAIT_CYCLES"]
    #[inline(always)]
    #[must_use]
    pub fn wait_cycles(&mut self) -> WaitCyclesW<SpiCtrlr0Spec> {
        WaitCyclesW::new(self, 11)
    }
}
#[doc = "SPI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_ctrlr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_ctrlr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiCtrlr0Spec;
impl crate::RegisterSpec for SpiCtrlr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ctrlr0::R`](R) reader structure"]
impl crate::Readable for SpiCtrlr0Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_ctrlr0::W`](W) writer structure"]
impl crate::Writable for SpiCtrlr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets spi_ctrlr0 to value 0"]
impl crate::Resettable for SpiCtrlr0Spec {
    const RESET_VALUE: u32 = 0;
}
