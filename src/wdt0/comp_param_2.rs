#[doc = "Register `comp_param_2` reader"]
pub type R = crate::R<CompParam2Spec>;
#[doc = "Register `comp_param_2` writer"]
pub type W = crate::W<CompParam2Spec>;
#[doc = "Field `cnt_rst` reader - cnt_rst"]
pub type CntRstR = crate::FieldReader<u32>;
#[doc = "Field `cnt_rst` writer - cnt_rst"]
pub type CntRstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cnt_rst"]
    #[inline(always)]
    pub fn cnt_rst(&self) -> CntRstR {
        CntRstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cnt_rst"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_rst(&mut self) -> CntRstW<CompParam2Spec> {
        CntRstW::new(self, 0)
    }
}
#[doc = "Component Parameters Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_param_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_param_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompParam2Spec;
impl crate::RegisterSpec for CompParam2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_param_2::R`](R) reader structure"]
impl crate::Readable for CompParam2Spec {}
#[doc = "`write(|w| ..)` method takes [`comp_param_2::W`](W) writer structure"]
impl crate::Writable for CompParam2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets comp_param_2 to value 0"]
impl crate::Resettable for CompParam2Spec {
    const RESET_VALUE: u32 = 0;
}
