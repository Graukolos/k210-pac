#[doc = "Register `clk_th2` reader"]
pub type R = crate::R<ClkTh2Spec>;
#[doc = "Register `clk_th2` writer"]
pub type W = crate::W<ClkTh2Spec>;
#[doc = "Field `timer0_clk` reader - "]
pub type Timer0ClkR = crate::FieldReader;
#[doc = "Field `timer0_clk` writer - "]
pub type Timer0ClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `timer1_clk` reader - "]
pub type Timer1ClkR = crate::FieldReader;
#[doc = "Field `timer1_clk` writer - "]
pub type Timer1ClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `timer2_clk` reader - "]
pub type Timer2ClkR = crate::FieldReader;
#[doc = "Field `timer2_clk` writer - "]
pub type Timer2ClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer0_clk(&self) -> Timer0ClkR {
        Timer0ClkR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn timer1_clk(&self) -> Timer1ClkR {
        Timer1ClkR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn timer2_clk(&self) -> Timer2ClkR {
        Timer2ClkR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_clk(&mut self) -> Timer0ClkW<ClkTh2Spec> {
        Timer0ClkW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_clk(&mut self) -> Timer1ClkW<ClkTh2Spec> {
        Timer1ClkW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_clk(&mut self) -> Timer2ClkW<ClkTh2Spec> {
        Timer2ClkW::new(self, 16)
    }
}
#[doc = "Clock threshold controller 2\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTh2Spec;
impl crate::RegisterSpec for ClkTh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_th2::R`](R) reader structure"]
impl crate::Readable for ClkTh2Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_th2::W`](W) writer structure"]
impl crate::Writable for ClkTh2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_th2 to value 0"]
impl crate::Resettable for ClkTh2Spec {
    const RESET_VALUE: u32 = 0;
}
