#[doc = "Register `clk_sel1` reader"]
pub type R = crate::R<ClkSel1Spec>;
#[doc = "Register `clk_sel1` writer"]
pub type W = crate::W<ClkSel1Spec>;
#[doc = "Field `spi3_sample_clk_sel` reader - "]
pub type Spi3SampleClkSelR = crate::BitReader;
#[doc = "Field `spi3_sample_clk_sel` writer - "]
pub type Spi3SampleClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi3_sample_clk_sel(&self) -> Spi3SampleClkSelR {
        Spi3SampleClkSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_sample_clk_sel(&mut self) -> Spi3SampleClkSelW<ClkSel1Spec> {
        Spi3SampleClkSelW::new(self, 0)
    }
}
#[doc = "Clock select controller 1\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkSel1Spec;
impl crate::RegisterSpec for ClkSel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_sel1::R`](R) reader structure"]
impl crate::Readable for ClkSel1Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_sel1::W`](W) writer structure"]
impl crate::Writable for ClkSel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_sel1 to value 0"]
impl crate::Resettable for ClkSel1Spec {
    const RESET_VALUE: u32 = 0;
}
