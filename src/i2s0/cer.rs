#[doc = "Register `cer` reader"]
pub type R = crate::R<CerSpec>;
#[doc = "Register `cer` writer"]
pub type W = crate::W<CerSpec>;
#[doc = "Field `clken` reader - Transmitter block enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `clken` writer - Transmitter block enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmitter block enable"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter block enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<CerSpec> {
        ClkenW::new(self, 0)
    }
}
#[doc = "Clock Generation enable\n\nYou can [`read`](crate::Reg::read) this register and get [`cer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CerSpec;
impl crate::RegisterSpec for CerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cer::R`](R) reader structure"]
impl crate::Readable for CerSpec {}
#[doc = "`write(|w| ..)` method takes [`cer::W`](W) writer structure"]
impl crate::Writable for CerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cer to value 0"]
impl crate::Resettable for CerSpec {
    const RESET_VALUE: u32 = 0;
}
