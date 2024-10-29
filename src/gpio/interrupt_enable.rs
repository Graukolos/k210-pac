#[doc = "Register `interrupt_enable` reader"]
pub type R = crate::R<InterruptEnableSpec>;
#[doc = "Register `interrupt_enable` writer"]
pub type W = crate::W<InterruptEnableSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt enable/disable registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptEnableSpec;
impl crate::RegisterSpec for InterruptEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enable::R`](R) reader structure"]
impl crate::Readable for InterruptEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enable::W`](W) writer structure"]
impl crate::Writable for InterruptEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets interrupt_enable to value 0"]
impl crate::Resettable for InterruptEnableSpec {
    const RESET_VALUE: u32 = 0;
}
