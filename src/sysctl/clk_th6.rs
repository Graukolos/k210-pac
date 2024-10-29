#[doc = "Register `clk_th6` reader"]
pub type R = crate::R<ClkTh6Spec>;
#[doc = "Register `clk_th6` writer"]
pub type W = crate::W<ClkTh6Spec>;
#[doc = "Field `wdt0_clk` reader - "]
pub type Wdt0ClkR = crate::FieldReader;
#[doc = "Field `wdt0_clk` writer - "]
pub type Wdt0ClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `wdt1_clk` reader - "]
pub type Wdt1ClkR = crate::FieldReader;
#[doc = "Field `wdt1_clk` writer - "]
pub type Wdt1ClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn wdt0_clk(&self) -> Wdt0ClkR {
        Wdt0ClkR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn wdt1_clk(&self) -> Wdt1ClkR {
        Wdt1ClkR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn wdt0_clk(&mut self) -> Wdt0ClkW<ClkTh6Spec> {
        Wdt0ClkW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn wdt1_clk(&mut self) -> Wdt1ClkW<ClkTh6Spec> {
        Wdt1ClkW::new(self, 8)
    }
}
#[doc = "Clock threshold controller 6\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTh6Spec;
impl crate::RegisterSpec for ClkTh6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_th6::R`](R) reader structure"]
impl crate::Readable for ClkTh6Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_th6::W`](W) writer structure"]
impl crate::Writable for ClkTh6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_th6 to value 0"]
impl crate::Resettable for ClkTh6Spec {
    const RESET_VALUE: u32 = 0;
}
