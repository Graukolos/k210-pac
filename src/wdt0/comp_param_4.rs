#[doc = "Register `comp_param_4` reader"]
pub type R = crate::R<CompParam4Spec>;
#[doc = "Register `comp_param_4` writer"]
pub type W = crate::W<CompParam4Spec>;
#[doc = "Field `user_top_init_max` reader - user_top_init_max"]
pub type UserTopInitMaxR = crate::FieldReader<u32>;
#[doc = "Field `user_top_init_max` writer - user_top_init_max"]
pub type UserTopInitMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - user_top_init_max"]
    #[inline(always)]
    pub fn user_top_init_max(&self) -> UserTopInitMaxR {
        UserTopInitMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - user_top_init_max"]
    #[inline(always)]
    #[must_use]
    pub fn user_top_init_max(&mut self) -> UserTopInitMaxW<CompParam4Spec> {
        UserTopInitMaxW::new(self, 0)
    }
}
#[doc = "Component Parameters Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_param_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_param_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompParam4Spec;
impl crate::RegisterSpec for CompParam4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_param_4::R`](R) reader structure"]
impl crate::Readable for CompParam4Spec {}
#[doc = "`write(|w| ..)` method takes [`comp_param_4::W`](W) writer structure"]
impl crate::Writable for CompParam4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets comp_param_4 to value 0"]
impl crate::Resettable for CompParam4Spec {
    const RESET_VALUE: u32 = 0;
}
