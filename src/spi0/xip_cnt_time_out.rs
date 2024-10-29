#[doc = "Register `xip_cnt_time_out` reader"]
pub type R = crate::R<XipCntTimeOutSpec>;
#[doc = "Register `xip_cnt_time_out` writer"]
pub type W = crate::W<XipCntTimeOutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "XIP time out register for continuous transfers\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_cnt_time_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_cnt_time_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XipCntTimeOutSpec;
impl crate::RegisterSpec for XipCntTimeOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_cnt_time_out::R`](R) reader structure"]
impl crate::Readable for XipCntTimeOutSpec {}
#[doc = "`write(|w| ..)` method takes [`xip_cnt_time_out::W`](W) writer structure"]
impl crate::Writable for XipCntTimeOutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets xip_cnt_time_out to value 0"]
impl crate::Resettable for XipCntTimeOutSpec {
    const RESET_VALUE: u32 = 0;
}
