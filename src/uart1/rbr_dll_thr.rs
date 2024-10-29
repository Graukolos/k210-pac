#[doc = "Register `rbr_dll_thr` reader"]
pub type R = crate::R<RbrDllThrSpec>;
#[doc = "Register `rbr_dll_thr` writer"]
pub type W = crate::W<RbrDllThrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Receive Buffer Register / Divisor Latch (Low) / Transmit Holding Register (depending on context and R/W)\n\nYou can [`read`](crate::Reg::read) this register and get [`rbr_dll_thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbr_dll_thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbrDllThrSpec;
impl crate::RegisterSpec for RbrDllThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbr_dll_thr::R`](R) reader structure"]
impl crate::Readable for RbrDllThrSpec {}
#[doc = "`write(|w| ..)` method takes [`rbr_dll_thr::W`](W) writer structure"]
impl crate::Writable for RbrDllThrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rbr_dll_thr to value 0"]
impl crate::Resettable for RbrDllThrSpec {
    const RESET_VALUE: u32 = 0;
}
