#[doc = "Register `srbr_sthr[%s]` reader"]
pub type R = crate::R<SrbrSthrSpec>;
#[doc = "Register `srbr_sthr[%s]` writer"]
pub type W = crate::W<SrbrSthrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Shadow Receive Buffer Register / Shadow Transmit Holding Register (depending on R/W)\n\nYou can [`read`](crate::Reg::read) this register and get [`srbr_sthr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srbr_sthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrbrSthrSpec;
impl crate::RegisterSpec for SrbrSthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srbr_sthr::R`](R) reader structure"]
impl crate::Readable for SrbrSthrSpec {}
#[doc = "`write(|w| ..)` method takes [`srbr_sthr::W`](W) writer structure"]
impl crate::Writable for SrbrSthrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets srbr_sthr[%s]
to value 0"]
impl crate::Resettable for SrbrSthrSpec {
    const RESET_VALUE: u32 = 0;
}
