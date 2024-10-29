#[doc = "Register `misc` reader"]
pub type R = crate::R<MiscSpec>;
#[doc = "Register `misc` writer"]
pub type W = crate::W<MiscSpec>;
#[doc = "Field `debug_sel` reader - "]
pub type DebugSelR = crate::FieldReader;
#[doc = "Field `debug_sel` writer - "]
pub type DebugSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `spi_dvp_data_enable` reader - "]
pub type SpiDvpDataEnableR = crate::BitReader;
#[doc = "Field `spi_dvp_data_enable` writer - "]
pub type SpiDvpDataEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn debug_sel(&self) -> DebugSelR {
        DebugSelR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_dvp_data_enable(&self) -> SpiDvpDataEnableR {
        SpiDvpDataEnableR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel(&mut self) -> DebugSelW<MiscSpec> {
        DebugSelW::new(self, 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn spi_dvp_data_enable(&mut self) -> SpiDvpDataEnableW<MiscSpec> {
        SpiDvpDataEnableW::new(self, 10)
    }
}
#[doc = "Miscellaneous controller\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscSpec;
impl crate::RegisterSpec for MiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MiscSpec {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MiscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets misc to value 0"]
impl crate::Resettable for MiscSpec {
    const RESET_VALUE: u32 = 0;
}
