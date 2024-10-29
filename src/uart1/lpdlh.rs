#[doc = "Register `lpdlh` reader"]
pub type R = crate::R<LpdlhSpec>;
#[doc = "Register `lpdlh` writer"]
pub type W = crate::W<LpdlhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Low Power Divisor Latch (High) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpdlh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdlh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpdlhSpec;
impl crate::RegisterSpec for LpdlhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdlh::R`](R) reader structure"]
impl crate::Readable for LpdlhSpec {}
#[doc = "`write(|w| ..)` method takes [`lpdlh::W`](W) writer structure"]
impl crate::Writable for LpdlhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lpdlh to value 0"]
impl crate::Resettable for LpdlhSpec {
    const RESET_VALUE: u32 = 0;
}
