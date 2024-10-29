#[doc = "Register `blk_tfr` reader"]
pub type R = crate::R<BlkTfrSpec>;
#[doc = "Register `blk_tfr` writer"]
pub type W = crate::W<BlkTfrSpec>;
#[doc = "Field `resumereq` reader - Block transfer resume request"]
pub type ResumereqR = crate::BitReader;
#[doc = "Field `resumereq` writer - Block transfer resume request"]
pub type ResumereqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Block transfer resume request"]
    #[inline(always)]
    pub fn resumereq(&self) -> ResumereqR {
        ResumereqR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Block transfer resume request"]
    #[inline(always)]
    #[must_use]
    pub fn resumereq(&mut self) -> ResumereqW<BlkTfrSpec> {
        ResumereqW::new(self, 0)
    }
}
#[doc = "Channel Block Transfer Resume Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_tfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_tfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkTfrSpec;
impl crate::RegisterSpec for BlkTfrSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`blk_tfr::R`](R) reader structure"]
impl crate::Readable for BlkTfrSpec {}
#[doc = "`write(|w| ..)` method takes [`blk_tfr::W`](W) writer structure"]
impl crate::Writable for BlkTfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets blk_tfr to value 0"]
impl crate::Resettable for BlkTfrSpec {
    const RESET_VALUE: u64 = 0;
}
