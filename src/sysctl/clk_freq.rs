#[doc = "Register `clk_freq` reader"]
pub type R = crate::R<ClkFreqSpec>;
#[doc = "Register `clk_freq` writer"]
pub type W = crate::W<ClkFreqSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "System clock base frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_freq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_freq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkFreqSpec;
impl crate::RegisterSpec for ClkFreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_freq::R`](R) reader structure"]
impl crate::Readable for ClkFreqSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_freq::W`](W) writer structure"]
impl crate::Writable for ClkFreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_freq to value 0"]
impl crate::Resettable for ClkFreqSpec {
    const RESET_VALUE: u32 = 0;
}
