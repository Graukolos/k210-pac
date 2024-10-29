#[doc = "Register `interrupt_debounce` reader"]
pub type R = crate::R<InterruptDebounceSpec>;
#[doc = "Register `interrupt_debounce` writer"]
pub type W = crate::W<InterruptDebounceSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt debounce registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_debounce::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_debounce::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptDebounceSpec;
impl crate::RegisterSpec for InterruptDebounceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_debounce::R`](R) reader structure"]
impl crate::Readable for InterruptDebounceSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_debounce::W`](W) writer structure"]
impl crate::Writable for InterruptDebounceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets interrupt_debounce to value 0"]
impl crate::Resettable for InterruptDebounceSpec {
    const RESET_VALUE: u32 = 0;
}
