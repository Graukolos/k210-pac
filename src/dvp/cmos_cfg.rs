#[doc = "Register `cmos_cfg` reader"]
pub type R = crate::R<CmosCfgSpec>;
#[doc = "Register `cmos_cfg` writer"]
pub type W = crate::W<CmosCfgSpec>;
#[doc = "Field `clk_div` reader - CLK_DIV"]
pub type ClkDivR = crate::FieldReader;
#[doc = "Field `clk_div` writer - CLK_DIV"]
pub type ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `clk_enable` reader - CLK_ENABLE"]
pub type ClkEnableR = crate::BitReader;
#[doc = "Field `clk_enable` writer - CLK_ENABLE"]
pub type ClkEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset` reader - RESET"]
pub type ResetR = crate::BitReader;
#[doc = "Field `reset` writer - RESET"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `power_down` reader - POWER_DOWN"]
pub type PowerDownR = crate::BitReader;
#[doc = "Field `power_down` writer - POWER_DOWN"]
pub type PowerDownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - CLK_DIV"]
    #[inline(always)]
    pub fn clk_div(&self) -> ClkDivR {
        ClkDivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - CLK_ENABLE"]
    #[inline(always)]
    pub fn clk_enable(&self) -> ClkEnableR {
        ClkEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - POWER_DOWN"]
    #[inline(always)]
    pub fn power_down(&self) -> PowerDownR {
        PowerDownR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLK_DIV"]
    #[inline(always)]
    #[must_use]
    pub fn clk_div(&mut self) -> ClkDivW<CmosCfgSpec> {
        ClkDivW::new(self, 0)
    }
    #[doc = "Bit 8 - CLK_ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn clk_enable(&mut self) -> ClkEnableW<CmosCfgSpec> {
        ClkEnableW::new(self, 8)
    }
    #[doc = "Bit 16 - RESET"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<CmosCfgSpec> {
        ResetW::new(self, 16)
    }
    #[doc = "Bit 24 - POWER_DOWN"]
    #[inline(always)]
    #[must_use]
    pub fn power_down(&mut self) -> PowerDownW<CmosCfgSpec> {
        PowerDownW::new(self, 24)
    }
}
#[doc = "CMOS Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmos_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmos_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmosCfgSpec;
impl crate::RegisterSpec for CmosCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmos_cfg::R`](R) reader structure"]
impl crate::Readable for CmosCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cmos_cfg::W`](W) writer structure"]
impl crate::Writable for CmosCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmos_cfg to value 0"]
impl crate::Resettable for CmosCfgSpec {
    const RESET_VALUE: u32 = 0;
}
