#[doc = "Register `clk_th3` reader"]
pub type R = crate::R<ClkTh3Spec>;
#[doc = "Register `clk_th3` writer"]
pub type W = crate::W<ClkTh3Spec>;
#[doc = "Field `i2s0_clk` reader - "]
pub type I2s0ClkR = crate::FieldReader<u16>;
#[doc = "Field `i2s0_clk` writer - "]
pub type I2s0ClkW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `i2s1_clk` reader - "]
pub type I2s1ClkR = crate::FieldReader<u16>;
#[doc = "Field `i2s1_clk` writer - "]
pub type I2s1ClkW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn i2s0_clk(&self) -> I2s0ClkR {
        I2s0ClkR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn i2s1_clk(&self) -> I2s1ClkR {
        I2s1ClkR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_clk(&mut self) -> I2s0ClkW<ClkTh3Spec> {
        I2s0ClkW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_clk(&mut self) -> I2s1ClkW<ClkTh3Spec> {
        I2s1ClkW::new(self, 16)
    }
}
#[doc = "Clock threshold controller 3\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTh3Spec;
impl crate::RegisterSpec for ClkTh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_th3::R`](R) reader structure"]
impl crate::Readable for ClkTh3Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_th3::W`](W) writer structure"]
impl crate::Writable for ClkTh3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_th3 to value 0"]
impl crate::Resettable for ClkTh3Spec {
    const RESET_VALUE: u32 = 0;
}
