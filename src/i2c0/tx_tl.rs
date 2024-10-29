#[doc = "Register `tx_tl` reader"]
pub type R = crate::R<TxTlSpec>;
#[doc = "Register `tx_tl` writer"]
pub type W = crate::W<TxTlSpec>;
#[doc = "Field `value` reader - VALUE"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `value` writer - VALUE"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - VALUE"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<TxTlSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_tl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_tl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxTlSpec;
impl crate::RegisterSpec for TxTlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_tl::R`](R) reader structure"]
impl crate::Readable for TxTlSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_tl::W`](W) writer structure"]
impl crate::Writable for TxTlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tx_tl to value 0"]
impl crate::Resettable for TxTlSpec {
    const RESET_VALUE: u32 = 0;
}
