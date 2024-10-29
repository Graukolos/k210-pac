#[doc = "Register `clk_th1` reader"]
pub type R = crate::R<ClkTh1Spec>;
#[doc = "Register `clk_th1` writer"]
pub type W = crate::W<ClkTh1Spec>;
#[doc = "Field `spi0_clk` reader - "]
pub type Spi0ClkR = crate::FieldReader;
#[doc = "Field `spi0_clk` writer - "]
pub type Spi0ClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `spi1_clk` reader - "]
pub type Spi1ClkR = crate::FieldReader;
#[doc = "Field `spi1_clk` writer - "]
pub type Spi1ClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `spi2_clk` reader - "]
pub type Spi2ClkR = crate::FieldReader;
#[doc = "Field `spi2_clk` writer - "]
pub type Spi2ClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `spi3_clk` reader - "]
pub type Spi3ClkR = crate::FieldReader;
#[doc = "Field `spi3_clk` writer - "]
pub type Spi3ClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn spi0_clk(&self) -> Spi0ClkR {
        Spi0ClkR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn spi1_clk(&self) -> Spi1ClkR {
        Spi1ClkR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn spi2_clk(&self) -> Spi2ClkR {
        Spi2ClkR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn spi3_clk(&self) -> Spi3ClkR {
        Spi3ClkR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn spi0_clk(&mut self) -> Spi0ClkW<ClkTh1Spec> {
        Spi0ClkW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_clk(&mut self) -> Spi1ClkW<ClkTh1Spec> {
        Spi1ClkW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_clk(&mut self) -> Spi2ClkW<ClkTh1Spec> {
        Spi2ClkW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_clk(&mut self) -> Spi3ClkW<ClkTh1Spec> {
        Spi3ClkW::new(self, 24)
    }
}
#[doc = "Clock threshold controller 1\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTh1Spec;
impl crate::RegisterSpec for ClkTh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_th1::R`](R) reader structure"]
impl crate::Readable for ClkTh1Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_th1::W`](W) writer structure"]
impl crate::Writable for ClkTh1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_th1 to value 0"]
impl crate::Resettable for ClkTh1Spec {
    const RESET_VALUE: u32 = 0;
}
