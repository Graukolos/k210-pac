#[doc = "Register `i2s_comp_param_1` reader"]
pub type R = crate::R<I2sCompParam1Spec>;
#[doc = "Register `i2s_comp_param_1` writer"]
pub type W = crate::W<I2sCompParam1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Component Parameter Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_comp_param_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_comp_param_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sCompParam1Spec;
impl crate::RegisterSpec for I2sCompParam1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_comp_param_1::R`](R) reader structure"]
impl crate::Readable for I2sCompParam1Spec {}
#[doc = "`write(|w| ..)` method takes [`i2s_comp_param_1::W`](W) writer structure"]
impl crate::Writable for I2sCompParam1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets i2s_comp_param_1 to value 0"]
impl crate::Resettable for I2sCompParam1Spec {
    const RESET_VALUE: u32 = 0;
}