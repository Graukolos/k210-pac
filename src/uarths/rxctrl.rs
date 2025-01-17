#[doc = "Register `rxctrl` reader"]
pub type R = crate::R<RxctrlSpec>;
#[doc = "Register `rxctrl` writer"]
pub type W = crate::W<RxctrlSpec>;
#[doc = "Field `rxen` reader - Receive enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `rxen` writer - Receive enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxcnt` reader - Receive watermark level"]
pub type RxcntR = crate::FieldReader;
#[doc = "Field `rxcnt` writer - Receive watermark level"]
pub type RxcntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Receive enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:18 - Receive watermark level"]
    #[inline(always)]
    pub fn rxcnt(&self) -> RxcntR {
        RxcntR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RxenW<RxctrlSpec> {
        RxenW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Receive watermark level"]
    #[inline(always)]
    #[must_use]
    pub fn rxcnt(&mut self) -> RxcntW<RxctrlSpec> {
        RxcntW::new(self, 16)
    }
}
#[doc = "Receive Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxctrlSpec;
impl crate::RegisterSpec for RxctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrl::R`](R) reader structure"]
impl crate::Readable for RxctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rxctrl::W`](W) writer structure"]
impl crate::Writable for RxctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rxctrl to value 0"]
impl crate::Resettable for RxctrlSpec {
    const RESET_VALUE: u32 = 0;
}
