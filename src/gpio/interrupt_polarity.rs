#[doc = "Register `interrupt_polarity` reader"]
pub type R = crate::R<InterruptPolaritySpec>;
#[doc = "Register `interrupt_polarity` writer"]
pub type W = crate::W<InterruptPolaritySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt polarity registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_polarity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_polarity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptPolaritySpec;
impl crate::RegisterSpec for InterruptPolaritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_polarity::R`](R) reader structure"]
impl crate::Readable for InterruptPolaritySpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_polarity::W`](W) writer structure"]
impl crate::Writable for InterruptPolaritySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets interrupt_polarity to value 0"]
impl crate::Resettable for InterruptPolaritySpec {
    const RESET_VALUE: u32 = 0;
}
