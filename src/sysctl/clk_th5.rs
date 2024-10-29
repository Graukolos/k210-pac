#[doc = "Register `clk_th5` reader"]
pub type R = crate::R<ClkTh5Spec>;
#[doc = "Register `clk_th5` writer"]
pub type W = crate::W<ClkTh5Spec>;
#[doc = "Field `i2s2_mclk` reader - "]
pub type I2s2MclkR = crate::FieldReader;
#[doc = "Field `i2s2_mclk` writer - "]
pub type I2s2MclkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `i2c0_clk` reader - "]
pub type I2c0ClkR = crate::FieldReader;
#[doc = "Field `i2c0_clk` writer - "]
pub type I2c0ClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `i2c1_clk` reader - "]
pub type I2c1ClkR = crate::FieldReader;
#[doc = "Field `i2c1_clk` writer - "]
pub type I2c1ClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `i2c2_clk` reader - "]
pub type I2c2ClkR = crate::FieldReader;
#[doc = "Field `i2c2_clk` writer - "]
pub type I2c2ClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2s2_mclk(&self) -> I2s2MclkR {
        I2s2MclkR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn i2c0_clk(&self) -> I2c0ClkR {
        I2c0ClkR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c1_clk(&self) -> I2c1ClkR {
        I2c1ClkR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn i2c2_clk(&self) -> I2c2ClkR {
        I2c2ClkR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2_mclk(&mut self) -> I2s2MclkW<ClkTh5Spec> {
        I2s2MclkW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_clk(&mut self) -> I2c0ClkW<ClkTh5Spec> {
        I2c0ClkW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_clk(&mut self) -> I2c1ClkW<ClkTh5Spec> {
        I2c1ClkW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_clk(&mut self) -> I2c2ClkW<ClkTh5Spec> {
        I2c2ClkW::new(self, 24)
    }
}
#[doc = "Clock threshold controller 5\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTh5Spec;
impl crate::RegisterSpec for ClkTh5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_th5::R`](R) reader structure"]
impl crate::Readable for ClkTh5Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_th5::W`](W) writer structure"]
impl crate::Writable for ClkTh5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_th5 to value 0"]
impl crate::Resettable for ClkTh5Spec {
    const RESET_VALUE: u32 = 0;
}
