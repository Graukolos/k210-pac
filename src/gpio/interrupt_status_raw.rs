#[doc = "Register `interrupt_status_raw` reader"]
pub type R = crate::R<InterruptStatusRawSpec>;
#[doc = "Register `interrupt_status_raw` writer"]
pub type W = crate::W<InterruptStatusRawSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Raw interrupt status registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_status_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptStatusRawSpec;
impl crate::RegisterSpec for InterruptStatusRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_status_raw::R`](R) reader structure"]
impl crate::Readable for InterruptStatusRawSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_status_raw::W`](W) writer structure"]
impl crate::Writable for InterruptStatusRawSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets interrupt_status_raw to value 0"]
impl crate::Resettable for InterruptStatusRawSpec {
    const RESET_VALUE: u32 = 0;
}
