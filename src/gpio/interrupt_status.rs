#[doc = "Register `interrupt_status` reader"]
pub type R = crate::R<InterruptStatusSpec>;
#[doc = "Register `interrupt_status` writer"]
pub type W = crate::W<InterruptStatusSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt status registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptStatusSpec;
impl crate::RegisterSpec for InterruptStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_status::R`](R) reader structure"]
impl crate::Readable for InterruptStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_status::W`](W) writer structure"]
impl crate::Writable for InterruptStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets interrupt_status to value 0"]
impl crate::Resettable for InterruptStatusSpec {
    const RESET_VALUE: u32 = 0;
}
