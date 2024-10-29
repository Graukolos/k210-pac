#[doc = "Register `irer` reader"]
pub type R = crate::R<IrerSpec>;
#[doc = "Register `irer` writer"]
pub type W = crate::W<IrerSpec>;
#[doc = "Field `rxen` reader - Receiver block enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `rxen` writer - Receiver block enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receiver block enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver block enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RxenW<IrerSpec> {
        RxenW::new(self, 0)
    }
}
#[doc = "Receiver Block Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrerSpec;
impl crate::RegisterSpec for IrerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irer::R`](R) reader structure"]
impl crate::Readable for IrerSpec {}
#[doc = "`write(|w| ..)` method takes [`irer::W`](W) writer structure"]
impl crate::Writable for IrerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets irer to value 0"]
impl crate::Resettable for IrerSpec {
    const RESET_VALUE: u32 = 0;
}
