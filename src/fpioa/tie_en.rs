#[doc = "Register `tie_en[%s]` reader"]
pub type R = crate::R<TieEnSpec>;
#[doc = "Register `tie_en[%s]` writer"]
pub type W = crate::W<TieEnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FPIOA GPIO multiplexer tie enable array\n\nYou can [`read`](crate::Reg::read) this register and get [`tie_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tie_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TieEnSpec;
impl crate::RegisterSpec for TieEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tie_en::R`](R) reader structure"]
impl crate::Readable for TieEnSpec {}
#[doc = "`write(|w| ..)` method takes [`tie_en::W`](W) writer structure"]
impl crate::Writable for TieEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tie_en[%s]
to value 0"]
impl crate::Resettable for TieEnSpec {
    const RESET_VALUE: u32 = 0;
}
