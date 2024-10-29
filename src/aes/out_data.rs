#[doc = "Register `out_data` reader"]
pub type R = crate::R<OutDataSpec>;
#[doc = "Register `out_data` writer"]
pub type W = crate::W<OutDataSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Plaintext/ciphertext output data\n\nYou can [`read`](crate::Reg::read) this register and get [`out_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutDataSpec;
impl crate::RegisterSpec for OutDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_data::R`](R) reader structure"]
impl crate::Readable for OutDataSpec {}
#[doc = "`write(|w| ..)` method takes [`out_data::W`](W) writer structure"]
impl crate::Writable for OutDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets out_data to value 0"]
impl crate::Resettable for OutDataSpec {
    const RESET_VALUE: u32 = 0;
}
