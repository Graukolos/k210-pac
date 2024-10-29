#[doc = "Register `xip_incr_inst` reader"]
pub type R = crate::R<XipIncrInstSpec>;
#[doc = "Register `xip_incr_inst` writer"]
pub type W = crate::W<XipIncrInstSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "XIP INCR transfer opcode\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_incr_inst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_incr_inst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XipIncrInstSpec;
impl crate::RegisterSpec for XipIncrInstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_incr_inst::R`](R) reader structure"]
impl crate::Readable for XipIncrInstSpec {}
#[doc = "`write(|w| ..)` method takes [`xip_incr_inst::W`](W) writer structure"]
impl crate::Writable for XipIncrInstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets xip_incr_inst to value 0"]
impl crate::Resettable for XipIncrInstSpec {
    const RESET_VALUE: u32 = 0;
}
