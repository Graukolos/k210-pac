#[doc = "Register `txctrl` reader"]
pub type R = crate::R<TxctrlSpec>;
#[doc = "Register `txctrl` writer"]
pub type W = crate::W<TxctrlSpec>;
#[doc = "Field `txen` reader - Transmit enable"]
pub type TxenR = crate::BitReader;
#[doc = "Field `txen` writer - Transmit enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nstop` reader - Number of stop bits"]
pub type NstopR = crate::BitReader;
#[doc = "Field `nstop` writer - Number of stop bits"]
pub type NstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txcnt` reader - Transmit watermark level"]
pub type TxcntR = crate::FieldReader;
#[doc = "Field `txcnt` writer - Transmit watermark level"]
pub type TxcntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Number of stop bits"]
    #[inline(always)]
    pub fn nstop(&self) -> NstopR {
        NstopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Transmit watermark level"]
    #[inline(always)]
    pub fn txcnt(&self) -> TxcntR {
        TxcntR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TxenW<TxctrlSpec> {
        TxenW::new(self, 0)
    }
    #[doc = "Bit 1 - Number of stop bits"]
    #[inline(always)]
    #[must_use]
    pub fn nstop(&mut self) -> NstopW<TxctrlSpec> {
        NstopW::new(self, 1)
    }
    #[doc = "Bits 16:18 - Transmit watermark level"]
    #[inline(always)]
    #[must_use]
    pub fn txcnt(&mut self) -> TxcntW<TxctrlSpec> {
        TxcntW::new(self, 16)
    }
}
#[doc = "Transmit Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxctrlSpec;
impl crate::RegisterSpec for TxctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrl::R`](R) reader structure"]
impl crate::Readable for TxctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`txctrl::W`](W) writer structure"]
impl crate::Writable for TxctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets txctrl to value 0"]
impl crate::Resettable for TxctrlSpec {
    const RESET_VALUE: u32 = 0;
}
