#[doc = "Register `tar` reader"]
pub type R = crate::R<TarSpec>;
#[doc = "Register `tar` writer"]
pub type W = crate::W<TarSpec>;
#[doc = "Field `address` reader - Target Address"]
pub type AddressR = crate::FieldReader<u16>;
#[doc = "Field `address` writer - Target Address"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `gc` reader - GC_OR_START"]
pub type GcR = crate::BitReader;
#[doc = "Field `gc` writer - GC_OR_START"]
pub type GcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `special` reader - SPECIAL"]
pub type SpecialR = crate::BitReader;
#[doc = "Field `special` writer - SPECIAL"]
pub type SpecialW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Master Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrMasterWidth {
    #[doc = "0: 7-bit address"]
    B7 = 0,
    #[doc = "1: 10-bit address"]
    B10 = 1,
}
impl From<AddrMasterWidth> for bool {
    #[inline(always)]
    fn from(variant: AddrMasterWidth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `addr_master_width` reader - Master Address"]
pub type AddrMasterWidthR = crate::BitReader<AddrMasterWidth>;
impl AddrMasterWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddrMasterWidth {
        match self.bits {
            false => AddrMasterWidth::B7,
            true => AddrMasterWidth::B10,
        }
    }
    #[doc = "7-bit address"]
    #[inline(always)]
    pub fn is_b7(&self) -> bool {
        *self == AddrMasterWidth::B7
    }
    #[doc = "10-bit address"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AddrMasterWidth::B10
    }
}
#[doc = "Field `addr_master_width` writer - Master Address"]
pub type AddrMasterWidthW<'a, REG> = crate::BitWriter<'a, REG, AddrMasterWidth>;
impl<'a, REG> AddrMasterWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit address"]
    #[inline(always)]
    pub fn b7(self) -> &'a mut crate::W<REG> {
        self.variant(AddrMasterWidth::B7)
    }
    #[doc = "10-bit address"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AddrMasterWidth::B10)
    }
}
impl R {
    #[doc = "Bits 0:9 - Target Address"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - GC_OR_START"]
    #[inline(always)]
    pub fn gc(&self) -> GcR {
        GcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SPECIAL"]
    #[inline(always)]
    pub fn special(&self) -> SpecialR {
        SpecialR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Master Address"]
    #[inline(always)]
    pub fn addr_master_width(&self) -> AddrMasterWidthR {
        AddrMasterWidthR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Target Address"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<TarSpec> {
        AddressW::new(self, 0)
    }
    #[doc = "Bit 10 - GC_OR_START"]
    #[inline(always)]
    #[must_use]
    pub fn gc(&mut self) -> GcW<TarSpec> {
        GcW::new(self, 10)
    }
    #[doc = "Bit 11 - SPECIAL"]
    #[inline(always)]
    #[must_use]
    pub fn special(&mut self) -> SpecialW<TarSpec> {
        SpecialW::new(self, 11)
    }
    #[doc = "Bit 12 - Master Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr_master_width(&mut self) -> AddrMasterWidthW<TarSpec> {
        AddrMasterWidthW::new(self, 12)
    }
}
#[doc = "Target Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TarSpec;
impl crate::RegisterSpec for TarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar::R`](R) reader structure"]
impl crate::Readable for TarSpec {}
#[doc = "`write(|w| ..)` method takes [`tar::W`](W) writer structure"]
impl crate::Writable for TarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tar to value 0"]
impl crate::Resettable for TarSpec {
    const RESET_VALUE: u32 = 0;
}
