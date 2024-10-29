#[doc = "Register `rff` reader"]
pub type R = crate::R<RffSpec>;
#[doc = "Register `rff` writer"]
pub type W = crate::W<RffSpec>;
#[doc = "Receiver channel FIFO reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flush {
    #[doc = "0: Not flush an individual FIFO"]
    NotFlush = 0,
    #[doc = "1: Flush an indiviadual FIFO"]
    Flush = 1,
}
impl From<Flush> for bool {
    #[inline(always)]
    fn from(variant: Flush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rxchfr` reader - Receiver channel FIFO reset"]
pub type RxchfrR = crate::BitReader<Flush>;
impl RxchfrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flush {
        match self.bits {
            false => Flush::NotFlush,
            true => Flush::Flush,
        }
    }
    #[doc = "Not flush an individual FIFO"]
    #[inline(always)]
    pub fn is_not_flush(&self) -> bool {
        *self == Flush::NotFlush
    }
    #[doc = "Flush an indiviadual FIFO"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == Flush::Flush
    }
}
#[doc = "Field `rxchfr` writer - Receiver channel FIFO reset"]
pub type RxchfrW<'a, REG> = crate::BitWriter<'a, REG, Flush>;
impl<'a, REG> RxchfrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not flush an individual FIFO"]
    #[inline(always)]
    pub fn not_flush(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::NotFlush)
    }
    #[doc = "Flush an indiviadual FIFO"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::Flush)
    }
}
impl R {
    #[doc = "Bit 0 - Receiver channel FIFO reset"]
    #[inline(always)]
    pub fn rxchfr(&self) -> RxchfrR {
        RxchfrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver channel FIFO reset"]
    #[inline(always)]
    #[must_use]
    pub fn rxchfr(&mut self) -> RxchfrW<RffSpec> {
        RxchfrW::new(self, 0)
    }
}
#[doc = "Receive FIFO Flush Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RffSpec;
impl crate::RegisterSpec for RffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rff::R`](R) reader structure"]
impl crate::Readable for RffSpec {}
#[doc = "`write(|w| ..)` method takes [`rff::W`](W) writer structure"]
impl crate::Writable for RffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rff to value 0"]
impl crate::Resettable for RffSpec {
    const RESET_VALUE: u32 = 0;
}
