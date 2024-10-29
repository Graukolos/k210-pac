#[doc = "Register `srt` reader"]
pub type R = crate::R<SrtSpec>;
#[doc = "Register `srt` writer"]
pub type W = crate::W<SrtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Shadow RCVR Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrtSpec;
impl crate::RegisterSpec for SrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srt::R`](R) reader structure"]
impl crate::Readable for SrtSpec {}
#[doc = "`write(|w| ..)` method takes [`srt::W`](W) writer structure"]
impl crate::Writable for SrtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets srt to value 0"]
impl crate::Resettable for SrtSpec {
    const RESET_VALUE: u32 = 0;
}
