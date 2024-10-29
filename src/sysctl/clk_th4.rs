#[doc = "Register `clk_th4` reader"]
pub type R = crate::R<ClkTh4Spec>;
#[doc = "Register `clk_th4` writer"]
pub type W = crate::W<ClkTh4Spec>;
#[doc = "Field `i2s2_clk` reader - "]
pub type I2s2ClkR = crate::FieldReader<u16>;
#[doc = "Field `i2s2_clk` writer - "]
pub type I2s2ClkW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `i2s0_mclk` reader - "]
pub type I2s0MclkR = crate::FieldReader;
#[doc = "Field `i2s0_mclk` writer - "]
pub type I2s0MclkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `i2s1_mclk` reader - "]
pub type I2s1MclkR = crate::FieldReader;
#[doc = "Field `i2s1_mclk` writer - "]
pub type I2s1MclkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn i2s2_clk(&self) -> I2s2ClkR {
        I2s2ClkR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2s0_mclk(&self) -> I2s0MclkR {
        I2s0MclkR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn i2s1_mclk(&self) -> I2s1MclkR {
        I2s1MclkR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2_clk(&mut self) -> I2s2ClkW<ClkTh4Spec> {
        I2s2ClkW::new(self, 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_mclk(&mut self) -> I2s0MclkW<ClkTh4Spec> {
        I2s0MclkW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_mclk(&mut self) -> I2s1MclkW<ClkTh4Spec> {
        I2s1MclkW::new(self, 24)
    }
}
#[doc = "Clock threshold controller 4\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTh4Spec;
impl crate::RegisterSpec for ClkTh4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_th4::R`](R) reader structure"]
impl crate::Readable for ClkTh4Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_th4::W`](W) writer structure"]
impl crate::Writable for ClkTh4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_th4 to value 0"]
impl crate::Resettable for ClkTh4Spec {
    const RESET_VALUE: u32 = 0;
}
