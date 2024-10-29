#[doc = "Register `text_data` reader"]
pub type R = crate::R<TextDataSpec>;
#[doc = "Register `text_data` writer"]
pub type W = crate::W<TextDataSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Plaintext/ciphertext input data\n\nYou can [`read`](crate::Reg::read) this register and get [`text_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`text_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TextDataSpec;
impl crate::RegisterSpec for TextDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`text_data::R`](R) reader structure"]
impl crate::Readable for TextDataSpec {}
#[doc = "`write(|w| ..)` method takes [`text_data::W`](W) writer structure"]
impl crate::Writable for TextDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets text_data to value 0"]
impl crate::Resettable for TextDataSpec {
    const RESET_VALUE: u32 = 0;
}
