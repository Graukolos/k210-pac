#[doc = "Register `lpdll` reader"]
pub type R = crate::R<LpdllSpec>;
#[doc = "Register `lpdll` writer"]
pub type W = crate::W<LpdllSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Low Power Divisor Latch (Low) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpdll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpdllSpec;
impl crate::RegisterSpec for LpdllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdll::R`](R) reader structure"]
impl crate::Readable for LpdllSpec {}
#[doc = "`write(|w| ..)` method takes [`lpdll::W`](W) writer structure"]
impl crate::Writable for LpdllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lpdll to value 0"]
impl crate::Resettable for LpdllSpec {
    const RESET_VALUE: u32 = 0;
}
