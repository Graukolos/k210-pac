#[doc = "Register `interrupt_level` reader"]
pub type R = crate::R<InterruptLevelSpec>;
#[doc = "Register `interrupt_level` writer"]
pub type W = crate::W<InterruptLevelSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt level registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_level::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_level::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptLevelSpec;
impl crate::RegisterSpec for InterruptLevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_level::R`](R) reader structure"]
impl crate::Readable for InterruptLevelSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_level::W`](W) writer structure"]
impl crate::Writable for InterruptLevelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets interrupt_level to value 0"]
impl crate::Resettable for InterruptLevelSpec {
    const RESET_VALUE: u32 = 0;
}
