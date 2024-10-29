#[doc = "Register `tie_val[%s]` reader"]
pub type R = crate::R<TieValSpec>;
#[doc = "Register `tie_val[%s]` writer"]
pub type W = crate::W<TieValSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FPIOA GPIO multiplexer tie value array\n\nYou can [`read`](crate::Reg::read) this register and get [`tie_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tie_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TieValSpec;
impl crate::RegisterSpec for TieValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tie_val::R`](R) reader structure"]
impl crate::Readable for TieValSpec {}
#[doc = "`write(|w| ..)` method takes [`tie_val::W`](W) writer structure"]
impl crate::Writable for TieValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tie_val[%s]
to value 0"]
impl crate::Resettable for TieValSpec {
    const RESET_VALUE: u32 = 0;
}
