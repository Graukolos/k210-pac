#[doc = "Register `sda_hold` reader"]
pub type R = crate::R<SdaHoldSpec>;
#[doc = "Register `sda_hold` writer"]
pub type W = crate::W<SdaHoldSpec>;
#[doc = "Field `tx` reader - TX"]
pub type TxR = crate::FieldReader<u16>;
#[doc = "Field `tx` writer - TX"]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rx` reader - RX"]
pub type RxR = crate::FieldReader;
#[doc = "Field `rx` writer - RX"]
pub type RxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - TX"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - RX"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - TX"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TxW<SdaHoldSpec> {
        TxW::new(self, 0)
    }
    #[doc = "Bits 16:23 - RX"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RxW<SdaHoldSpec> {
        RxW::new(self, 16)
    }
}
#[doc = "SDA Hold Time Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sda_hold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sda_hold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdaHoldSpec;
impl crate::RegisterSpec for SdaHoldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda_hold::R`](R) reader structure"]
impl crate::Readable for SdaHoldSpec {}
#[doc = "`write(|w| ..)` method takes [`sda_hold::W`](W) writer structure"]
impl crate::Writable for SdaHoldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sda_hold to value 0"]
impl crate::Resettable for SdaHoldSpec {
    const RESET_VALUE: u32 = 0;
}
