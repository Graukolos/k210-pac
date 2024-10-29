#[doc = "Register `torr` reader"]
pub type R = crate::R<TorrSpec>;
#[doc = "Register `torr` writer"]
pub type W = crate::W<TorrSpec>;
#[doc = "Field `top0` reader - top (lower half)"]
pub type Top0R = crate::FieldReader;
#[doc = "Field `top0` writer - top (lower half)"]
pub type Top0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `top1` reader - top (upper half)"]
pub type Top1R = crate::FieldReader;
#[doc = "Field `top1` writer - top (upper half)"]
pub type Top1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - top (lower half)"]
    #[inline(always)]
    pub fn top0(&self) -> Top0R {
        Top0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - top (upper half)"]
    #[inline(always)]
    pub fn top1(&self) -> Top1R {
        Top1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - top (lower half)"]
    #[inline(always)]
    #[must_use]
    pub fn top0(&mut self) -> Top0W<TorrSpec> {
        Top0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - top (upper half)"]
    #[inline(always)]
    #[must_use]
    pub fn top1(&mut self) -> Top1W<TorrSpec> {
        Top1W::new(self, 4)
    }
}
#[doc = "Timeout Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`torr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`torr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TorrSpec;
impl crate::RegisterSpec for TorrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`torr::R`](R) reader structure"]
impl crate::Readable for TorrSpec {}
#[doc = "`write(|w| ..)` method takes [`torr::W`](W) writer structure"]
impl crate::Writable for TorrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets torr to value 0"]
impl crate::Resettable for TorrSpec {
    const RESET_VALUE: u32 = 0;
}
