#[doc = "Register `pll0` reader"]
pub type R = crate::R<Pll0Spec>;
#[doc = "Register `pll0` writer"]
pub type W = crate::W<Pll0Spec>;
#[doc = "Field `clkr` reader - "]
pub type ClkrR = crate::FieldReader;
#[doc = "Field `clkr` writer - "]
pub type ClkrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `clkf` reader - "]
pub type ClkfR = crate::FieldReader;
#[doc = "Field `clkf` writer - "]
pub type ClkfW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `clkod` reader - "]
pub type ClkodR = crate::FieldReader;
#[doc = "Field `clkod` writer - "]
pub type ClkodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bwadj` reader - "]
pub type BwadjR = crate::FieldReader;
#[doc = "Field `bwadj` writer - "]
pub type BwadjW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `reset` reader - "]
pub type ResetR = crate::BitReader;
#[doc = "Field `reset` writer - "]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwrd` reader - "]
pub type PwrdR = crate::BitReader;
#[doc = "Field `pwrd` writer - "]
pub type PwrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intfb` reader - "]
pub type IntfbR = crate::BitReader;
#[doc = "Field `intfb` writer - "]
pub type IntfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bypass` reader - "]
pub type BypassR = crate::BitReader;
#[doc = "Field `bypass` writer - "]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test` reader - "]
pub type TestR = crate::BitReader;
#[doc = "Field `test` writer - "]
pub type TestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `out_en` reader - "]
pub type OutEnR = crate::BitReader;
#[doc = "Field `out_en` writer - "]
pub type OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_en` reader - "]
pub type TestEnR = crate::BitReader;
#[doc = "Field `test_en` writer - "]
pub type TestEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn clkr(&self) -> ClkrR {
        ClkrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    pub fn clkf(&self) -> ClkfR {
        ClkfR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn clkod(&self) -> ClkodR {
        ClkodR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn bwadj(&self) -> BwadjR {
        BwadjR::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pwrd(&self) -> PwrdR {
        PwrdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn intfb(&self) -> IntfbR {
        IntfbR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn out_en(&self) -> OutEnR {
        OutEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn test_en(&self) -> TestEnR {
        TestEnR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn clkr(&mut self) -> ClkrW<Pll0Spec> {
        ClkrW::new(self, 0)
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    #[must_use]
    pub fn clkf(&mut self) -> ClkfW<Pll0Spec> {
        ClkfW::new(self, 4)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    #[must_use]
    pub fn clkod(&mut self) -> ClkodW<Pll0Spec> {
        ClkodW::new(self, 10)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    #[must_use]
    pub fn bwadj(&mut self) -> BwadjW<Pll0Spec> {
        BwadjW::new(self, 14)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<Pll0Spec> {
        ResetW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pwrd(&mut self) -> PwrdW<Pll0Spec> {
        PwrdW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn intfb(&mut self) -> IntfbW<Pll0Spec> {
        IntfbW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<Pll0Spec> {
        BypassW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TestW<Pll0Spec> {
        TestW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn out_en(&mut self) -> OutEnW<Pll0Spec> {
        OutEnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn test_en(&mut self) -> TestEnW<Pll0Spec> {
        TestEnW::new(self, 26)
    }
}
#[doc = "PLL0 controller\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0Spec;
impl crate::RegisterSpec for Pll0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0::R`](R) reader structure"]
impl crate::Readable for Pll0Spec {}
#[doc = "`write(|w| ..)` method takes [`pll0::W`](W) writer structure"]
impl crate::Writable for Pll0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pll0 to value 0"]
impl crate::Resettable for Pll0Spec {
    const RESET_VALUE: u32 = 0;
}
