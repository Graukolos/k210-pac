#[doc = "Register `iter` reader"]
pub type R = crate::R<IterSpec>;
#[doc = "Register `iter` writer"]
pub type W = crate::W<IterSpec>;
#[doc = "Field `txen` reader - Transmitter block enable"]
pub type TxenR = crate::BitReader;
#[doc = "Field `txen` writer - Transmitter block enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmitter block enable"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter block enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TxenW<IterSpec> {
        TxenW::new(self, 0)
    }
}
#[doc = "Transmitter Block Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IterSpec;
impl crate::RegisterSpec for IterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iter::R`](R) reader structure"]
impl crate::Readable for IterSpec {}
#[doc = "`write(|w| ..)` method takes [`iter::W`](W) writer structure"]
impl crate::Writable for IterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets iter to value 0"]
impl crate::Resettable for IterSpec {
    const RESET_VALUE: u32 = 0;
}
