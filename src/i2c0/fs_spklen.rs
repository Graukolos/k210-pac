#[doc = "Register `fs_spklen` reader"]
pub type R = crate::R<FsSpklenSpec>;
#[doc = "Register `fs_spklen` writer"]
pub type W = crate::W<FsSpklenSpec>;
#[doc = "Field `value` reader - VALUE"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `value` writer - VALUE"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VALUE"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<FsSpklenSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "SS, FS or FM+ spike suppression limit\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_spklen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_spklen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsSpklenSpec;
impl crate::RegisterSpec for FsSpklenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_spklen::R`](R) reader structure"]
impl crate::Readable for FsSpklenSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_spklen::W`](W) writer structure"]
impl crate::Writable for FsSpklenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fs_spklen to value 0"]
impl crate::Resettable for FsSpklenSpec {
    const RESET_VALUE: u32 = 0;
}
