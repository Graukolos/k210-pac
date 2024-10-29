#[doc = "Register `general_call` reader"]
pub type R = crate::R<GeneralCallSpec>;
#[doc = "Register `general_call` writer"]
pub type W = crate::W<GeneralCallSpec>;
#[doc = "Field `call_enable` reader - CALL_ENABLE"]
pub type CallEnableR = crate::BitReader;
#[doc = "Field `call_enable` writer - CALL_ENABLE"]
pub type CallEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CALL_ENABLE"]
    #[inline(always)]
    pub fn call_enable(&self) -> CallEnableR {
        CallEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CALL_ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn call_enable(&mut self) -> CallEnableW<GeneralCallSpec> {
        CallEnableW::new(self, 0)
    }
}
#[doc = "ACK General Call Register\n\nYou can [`read`](crate::Reg::read) this register and get [`general_call::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`general_call::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GeneralCallSpec;
impl crate::RegisterSpec for GeneralCallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`general_call::R`](R) reader structure"]
impl crate::Readable for GeneralCallSpec {}
#[doc = "`write(|w| ..)` method takes [`general_call::W`](W) writer structure"]
impl crate::Writable for GeneralCallSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets general_call to value 0"]
impl crate::Resettable for GeneralCallSpec {
    const RESET_VALUE: u32 = 0;
}
