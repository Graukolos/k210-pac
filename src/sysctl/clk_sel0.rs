#[doc = "Register `clk_sel0` reader"]
pub type R = crate::R<ClkSel0Spec>;
#[doc = "Register `clk_sel0` writer"]
pub type W = crate::W<ClkSel0Spec>;
#[doc = "Field `aclk_sel` reader - "]
pub type AclkSelR = crate::BitReader;
#[doc = "Field `aclk_sel` writer - "]
pub type AclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aclk_divider_sel` reader - "]
pub type AclkDividerSelR = crate::FieldReader;
#[doc = "Field `aclk_divider_sel` writer - "]
pub type AclkDividerSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `apb0_clk_sel` reader - "]
pub type Apb0ClkSelR = crate::FieldReader;
#[doc = "Field `apb0_clk_sel` writer - "]
pub type Apb0ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `apb1_clk_sel` reader - "]
pub type Apb1ClkSelR = crate::FieldReader;
#[doc = "Field `apb1_clk_sel` writer - "]
pub type Apb1ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `apb2_clk_sel` reader - "]
pub type Apb2ClkSelR = crate::FieldReader;
#[doc = "Field `apb2_clk_sel` writer - "]
pub type Apb2ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `spi3_clk_sel` reader - "]
pub type Spi3ClkSelR = crate::BitReader;
#[doc = "Field `spi3_clk_sel` writer - "]
pub type Spi3ClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer0_clk_sel` reader - "]
pub type Timer0ClkSelR = crate::BitReader;
#[doc = "Field `timer0_clk_sel` writer - "]
pub type Timer0ClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer1_clk_sel` reader - "]
pub type Timer1ClkSelR = crate::BitReader;
#[doc = "Field `timer1_clk_sel` writer - "]
pub type Timer1ClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer2_clk_sel` reader - "]
pub type Timer2ClkSelR = crate::BitReader;
#[doc = "Field `timer2_clk_sel` writer - "]
pub type Timer2ClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aclk_sel(&self) -> AclkSelR {
        AclkSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn aclk_divider_sel(&self) -> AclkDividerSelR {
        AclkDividerSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn apb0_clk_sel(&self) -> Apb0ClkSelR {
        Apb0ClkSelR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn apb1_clk_sel(&self) -> Apb1ClkSelR {
        Apb1ClkSelR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn apb2_clk_sel(&self) -> Apb2ClkSelR {
        Apb2ClkSelR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi3_clk_sel(&self) -> Spi3ClkSelR {
        Spi3ClkSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timer0_clk_sel(&self) -> Timer0ClkSelR {
        Timer0ClkSelR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn timer1_clk_sel(&self) -> Timer1ClkSelR {
        Timer1ClkSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timer2_clk_sel(&self) -> Timer2ClkSelR {
        Timer2ClkSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_sel(&mut self) -> AclkSelW<ClkSel0Spec> {
        AclkSelW::new(self, 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_divider_sel(&mut self) -> AclkDividerSelW<ClkSel0Spec> {
        AclkDividerSelW::new(self, 1)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    #[must_use]
    pub fn apb0_clk_sel(&mut self) -> Apb0ClkSelW<ClkSel0Spec> {
        Apb0ClkSelW::new(self, 3)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    #[must_use]
    pub fn apb1_clk_sel(&mut self) -> Apb1ClkSelW<ClkSel0Spec> {
        Apb1ClkSelW::new(self, 6)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn apb2_clk_sel(&mut self) -> Apb2ClkSelW<ClkSel0Spec> {
        Apb2ClkSelW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_clk_sel(&mut self) -> Spi3ClkSelW<ClkSel0Spec> {
        Spi3ClkSelW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_clk_sel(&mut self) -> Timer0ClkSelW<ClkSel0Spec> {
        Timer0ClkSelW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_clk_sel(&mut self) -> Timer1ClkSelW<ClkSel0Spec> {
        Timer1ClkSelW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_clk_sel(&mut self) -> Timer2ClkSelW<ClkSel0Spec> {
        Timer2ClkSelW::new(self, 15)
    }
}
#[doc = "Clock select controller 0\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_sel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_sel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkSel0Spec;
impl crate::RegisterSpec for ClkSel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_sel0::R`](R) reader structure"]
impl crate::Readable for ClkSel0Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_sel0::W`](W) writer structure"]
impl crate::Writable for ClkSel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_sel0 to value 0"]
impl crate::Resettable for ClkSel0Spec {
    const RESET_VALUE: u32 = 0;
}
