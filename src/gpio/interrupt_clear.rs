#[doc = "Register `interrupt_clear` reader"]
pub type R = crate::R<InterruptClearSpec>;
#[doc = "Register `interrupt_clear` writer"]
pub type W = crate::W<InterruptClearSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Registers for clearing interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptClearSpec;
impl crate::RegisterSpec for InterruptClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_clear::R`](R) reader structure"]
impl crate::Readable for InterruptClearSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_clear::W`](W) writer structure"]
impl crate::Writable for InterruptClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets interrupt_clear to value 0"]
impl crate::Resettable for InterruptClearSpec {
    const RESET_VALUE: u32 = 0;
}
