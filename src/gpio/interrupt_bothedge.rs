#[doc = "Register `interrupt_bothedge` reader"]
pub type R = crate::R<InterruptBothedgeSpec>;
#[doc = "Register `interrupt_bothedge` writer"]
pub type W = crate::W<InterruptBothedgeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt both edge type\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_bothedge::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_bothedge::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptBothedgeSpec;
impl crate::RegisterSpec for InterruptBothedgeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_bothedge::R`](R) reader structure"]
impl crate::Readable for InterruptBothedgeSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_bothedge::W`](W) writer structure"]
impl crate::Writable for InterruptBothedgeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets interrupt_bothedge to value 0"]
impl crate::Resettable for InterruptBothedgeSpec {
    const RESET_VALUE: u32 = 0;
}
