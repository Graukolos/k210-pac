#[doc = "Register `clk_th0` reader"]
pub type R = crate::R<ClkTh0Spec>;
#[doc = "Register `clk_th0` writer"]
pub type W = crate::W<ClkTh0Spec>;
#[doc = "Field `sram0_gclk` reader - "]
pub type Sram0GclkR = crate::FieldReader;
#[doc = "Field `sram0_gclk` writer - "]
pub type Sram0GclkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sram1_gclk` reader - "]
pub type Sram1GclkR = crate::FieldReader;
#[doc = "Field `sram1_gclk` writer - "]
pub type Sram1GclkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ai_gclk` reader - "]
pub type AiGclkR = crate::FieldReader;
#[doc = "Field `ai_gclk` writer - "]
pub type AiGclkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dvp_gclk` reader - "]
pub type DvpGclkR = crate::FieldReader;
#[doc = "Field `dvp_gclk` writer - "]
pub type DvpGclkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rom_gclk` reader - "]
pub type RomGclkR = crate::FieldReader;
#[doc = "Field `rom_gclk` writer - "]
pub type RomGclkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sram0_gclk(&self) -> Sram0GclkR {
        Sram0GclkR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn sram1_gclk(&self) -> Sram1GclkR {
        Sram1GclkR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ai_gclk(&self) -> AiGclkR {
        AiGclkR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn dvp_gclk(&self) -> DvpGclkR {
        DvpGclkR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rom_gclk(&self) -> RomGclkR {
        RomGclkR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn sram0_gclk(&mut self) -> Sram0GclkW<ClkTh0Spec> {
        Sram0GclkW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn sram1_gclk(&mut self) -> Sram1GclkW<ClkTh0Spec> {
        Sram1GclkW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn ai_gclk(&mut self) -> AiGclkW<ClkTh0Spec> {
        AiGclkW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn dvp_gclk(&mut self) -> DvpGclkW<ClkTh0Spec> {
        DvpGclkW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn rom_gclk(&mut self) -> RomGclkW<ClkTh0Spec> {
        RomGclkW::new(self, 16)
    }
}
#[doc = "Clock threshold controller 0\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTh0Spec;
impl crate::RegisterSpec for ClkTh0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_th0::R`](R) reader structure"]
impl crate::Readable for ClkTh0Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_th0::W`](W) writer structure"]
impl crate::Writable for ClkTh0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_th0 to value 0"]
impl crate::Resettable for ClkTh0Spec {
    const RESET_VALUE: u32 = 0;
}
