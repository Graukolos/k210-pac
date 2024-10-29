#[doc = "Register `spi_sleep` reader"]
pub type R = crate::R<SpiSleepSpec>;
#[doc = "Register `spi_sleep` writer"]
pub type W = crate::W<SpiSleepSpec>;
#[doc = "Field `ssi0_sleep` reader - "]
pub type Ssi0SleepR = crate::BitReader;
#[doc = "Field `ssi0_sleep` writer - "]
pub type Ssi0SleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ssi1_sleep` reader - "]
pub type Ssi1SleepR = crate::BitReader;
#[doc = "Field `ssi1_sleep` writer - "]
pub type Ssi1SleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ssi2_sleep` reader - "]
pub type Ssi2SleepR = crate::BitReader;
#[doc = "Field `ssi2_sleep` writer - "]
pub type Ssi2SleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ssi3_sleep` reader - "]
pub type Ssi3SleepR = crate::BitReader;
#[doc = "Field `ssi3_sleep` writer - "]
pub type Ssi3SleepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ssi0_sleep(&self) -> Ssi0SleepR {
        Ssi0SleepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ssi1_sleep(&self) -> Ssi1SleepR {
        Ssi1SleepR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ssi2_sleep(&self) -> Ssi2SleepR {
        Ssi2SleepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ssi3_sleep(&self) -> Ssi3SleepR {
        Ssi3SleepR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ssi0_sleep(&mut self) -> Ssi0SleepW<SpiSleepSpec> {
        Ssi0SleepW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ssi1_sleep(&mut self) -> Ssi1SleepW<SpiSleepSpec> {
        Ssi1SleepW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ssi2_sleep(&mut self) -> Ssi2SleepW<SpiSleepSpec> {
        Ssi2SleepW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ssi3_sleep(&mut self) -> Ssi3SleepW<SpiSleepSpec> {
        Ssi3SleepW::new(self, 3)
    }
}
#[doc = "SPI sleep controller\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sleep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sleep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSleepSpec;
impl crate::RegisterSpec for SpiSleepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_sleep::R`](R) reader structure"]
impl crate::Readable for SpiSleepSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_sleep::W`](W) writer structure"]
impl crate::Writable for SpiSleepSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets spi_sleep to value 0"]
impl crate::Resettable for SpiSleepSpec {
    const RESET_VALUE: u32 = 0;
}
