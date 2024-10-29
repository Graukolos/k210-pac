#[doc = "Register `comp_param_3` reader"]
pub type R = crate::R<CompParam3Spec>;
#[doc = "Register `comp_param_3` writer"]
pub type W = crate::W<CompParam3Spec>;
#[doc = "Field `top_rst` reader - top_rst"]
pub type TopRstR = crate::FieldReader<u32>;
#[doc = "Field `top_rst` writer - top_rst"]
pub type TopRstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - top_rst"]
    #[inline(always)]
    pub fn top_rst(&self) -> TopRstR {
        TopRstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - top_rst"]
    #[inline(always)]
    #[must_use]
    pub fn top_rst(&mut self) -> TopRstW<CompParam3Spec> {
        TopRstW::new(self, 0)
    }
}
#[doc = "Component Parameters Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_param_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_param_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompParam3Spec;
impl crate::RegisterSpec for CompParam3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_param_3::R`](R) reader structure"]
impl crate::Readable for CompParam3Spec {}
#[doc = "`write(|w| ..)` method takes [`comp_param_3::W`](W) writer structure"]
impl crate::Writable for CompParam3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets comp_param_3 to value 0"]
impl crate::Resettable for CompParam3Spec {
    const RESET_VALUE: u32 = 0;
}
