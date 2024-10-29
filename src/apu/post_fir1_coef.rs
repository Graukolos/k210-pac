#[doc = "Register `post_fir1_coef[%s]` reader"]
pub type R = crate::R<PostFir1CoefSpec>;
#[doc = "Register `post_fir1_coef[%s]` writer"]
pub type W = crate::W<PostFir1CoefSpec>;
#[doc = "Field `tap0` reader - Tap 0"]
pub type Tap0R = crate::FieldReader<u16>;
#[doc = "Field `tap0` writer - Tap 0"]
pub type Tap0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tap1` reader - Tap 1"]
pub type Tap1R = crate::FieldReader<u16>;
#[doc = "Field `tap1` writer - Tap 1"]
pub type Tap1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Tap 0"]
    #[inline(always)]
    pub fn tap0(&self) -> Tap0R {
        Tap0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Tap 1"]
    #[inline(always)]
    pub fn tap1(&self) -> Tap1R {
        Tap1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Tap 0"]
    #[inline(always)]
    #[must_use]
    pub fn tap0(&mut self) -> Tap0W<PostFir1CoefSpec> {
        Tap0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Tap 1"]
    #[inline(always)]
    #[must_use]
    pub fn tap1(&mut self) -> Tap1W<PostFir1CoefSpec> {
        Tap1W::new(self, 16)
    }
}
#[doc = "FIR1 post-filter coefficients\n\nYou can [`read`](crate::Reg::read) this register and get [`post_fir1_coef::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`post_fir1_coef::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PostFir1CoefSpec;
impl crate::RegisterSpec for PostFir1CoefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`post_fir1_coef::R`](R) reader structure"]
impl crate::Readable for PostFir1CoefSpec {}
#[doc = "`write(|w| ..)` method takes [`post_fir1_coef::W`](W) writer structure"]
impl crate::Writable for PostFir1CoefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets post_fir1_coef[%s]
to value 0"]
impl crate::Resettable for PostFir1CoefSpec {
    const RESET_VALUE: u32 = 0;
}
