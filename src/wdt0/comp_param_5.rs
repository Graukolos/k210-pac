#[doc = "Register `comp_param_5` reader"]
pub type R = crate::R<CompParam5Spec>;
#[doc = "Register `comp_param_5` writer"]
pub type W = crate::W<CompParam5Spec>;
#[doc = "Field `user_top_max` reader - user_top_max"]
pub type UserTopMaxR = crate::FieldReader<u32>;
#[doc = "Field `user_top_max` writer - user_top_max"]
pub type UserTopMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - user_top_max"]
    #[inline(always)]
    pub fn user_top_max(&self) -> UserTopMaxR {
        UserTopMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - user_top_max"]
    #[inline(always)]
    #[must_use]
    pub fn user_top_max(&mut self) -> UserTopMaxW<CompParam5Spec> {
        UserTopMaxW::new(self, 0)
    }
}
#[doc = "Component Parameters Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_param_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_param_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompParam5Spec;
impl crate::RegisterSpec for CompParam5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_param_5::R`](R) reader structure"]
impl crate::Readable for CompParam5Spec {}
#[doc = "`write(|w| ..)` method takes [`comp_param_5::W`](W) writer structure"]
impl crate::Writable for CompParam5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets comp_param_5 to value 0"]
impl crate::Resettable for CompParam5Spec {
    const RESET_VALUE: u32 = 0;
}
