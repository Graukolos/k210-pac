#[doc = "Register `dlh_ier` reader"]
pub type R = crate::R<DlhIerSpec>;
#[doc = "Register `dlh_ier` writer"]
pub type W = crate::W<DlhIerSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Divisor Latch (High) / Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dlh_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlh_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlhIerSpec;
impl crate::RegisterSpec for DlhIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlh_ier::R`](R) reader structure"]
impl crate::Readable for DlhIerSpec {}
#[doc = "`write(|w| ..)` method takes [`dlh_ier::W`](W) writer structure"]
impl crate::Writable for DlhIerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dlh_ier to value 0"]
impl crate::Resettable for DlhIerSpec {
    const RESET_VALUE: u32 = 0;
}
