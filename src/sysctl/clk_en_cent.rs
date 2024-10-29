#[doc = "Register `clk_en_cent` reader"]
pub type R = crate::R<ClkEnCentSpec>;
#[doc = "Register `clk_en_cent` writer"]
pub type W = crate::W<ClkEnCentSpec>;
#[doc = "Field `cpu_clk_en` reader - "]
pub type CpuClkEnR = crate::BitReader;
#[doc = "Field `cpu_clk_en` writer - "]
pub type CpuClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram0_clk_en` reader - "]
pub type Sram0ClkEnR = crate::BitReader;
#[doc = "Field `sram0_clk_en` writer - "]
pub type Sram0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram1_clk_en` reader - "]
pub type Sram1ClkEnR = crate::BitReader;
#[doc = "Field `sram1_clk_en` writer - "]
pub type Sram1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `apb0_clk_en` reader - "]
pub type Apb0ClkEnR = crate::BitReader;
#[doc = "Field `apb0_clk_en` writer - "]
pub type Apb0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `apb1_clk_en` reader - "]
pub type Apb1ClkEnR = crate::BitReader;
#[doc = "Field `apb1_clk_en` writer - "]
pub type Apb1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `apb2_clk_en` reader - "]
pub type Apb2ClkEnR = crate::BitReader;
#[doc = "Field `apb2_clk_en` writer - "]
pub type Apb2ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpu_clk_en(&self) -> CpuClkEnR {
        CpuClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sram0_clk_en(&self) -> Sram0ClkEnR {
        Sram0ClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sram1_clk_en(&self) -> Sram1ClkEnR {
        Sram1ClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn apb0_clk_en(&self) -> Apb0ClkEnR {
        Apb0ClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn apb1_clk_en(&self) -> Apb1ClkEnR {
        Apb1ClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn apb2_clk_en(&self) -> Apb2ClkEnR {
        Apb2ClkEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_clk_en(&mut self) -> CpuClkEnW<ClkEnCentSpec> {
        CpuClkEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sram0_clk_en(&mut self) -> Sram0ClkEnW<ClkEnCentSpec> {
        Sram0ClkEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sram1_clk_en(&mut self) -> Sram1ClkEnW<ClkEnCentSpec> {
        Sram1ClkEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn apb0_clk_en(&mut self) -> Apb0ClkEnW<ClkEnCentSpec> {
        Apb0ClkEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn apb1_clk_en(&mut self) -> Apb1ClkEnW<ClkEnCentSpec> {
        Apb1ClkEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn apb2_clk_en(&mut self) -> Apb2ClkEnW<ClkEnCentSpec> {
        Apb2ClkEnW::new(self, 5)
    }
}
#[doc = "Central clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en_cent::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en_cent::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkEnCentSpec;
impl crate::RegisterSpec for ClkEnCentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_en_cent::R`](R) reader structure"]
impl crate::Readable for ClkEnCentSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_en_cent::W`](W) writer structure"]
impl crate::Writable for ClkEnCentSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_en_cent to value 0"]
impl crate::Resettable for ClkEnCentSpec {
    const RESET_VALUE: u32 = 0;
}
