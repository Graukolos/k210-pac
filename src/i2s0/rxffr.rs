#[doc = "Register `rxffr` reader"]
pub type R = crate::R<RxffrSpec>;
#[doc = "Register `rxffr` writer"]
pub type W = crate::W<RxffrSpec>;
#[doc = "Receiver FIFO reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flush {
    #[doc = "0: Not flush FIFO"]
    NotFlush = 0,
    #[doc = "1: Flush FIFO"]
    Flush = 1,
}
impl From<Flush> for bool {
    #[inline(always)]
    fn from(variant: Flush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rxffr` reader - Receiver FIFO reset"]
pub type RxffrR = crate::BitReader<Flush>;
impl RxffrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flush {
        match self.bits {
            false => Flush::NotFlush,
            true => Flush::Flush,
        }
    }
    #[doc = "Not flush FIFO"]
    #[inline(always)]
    pub fn is_not_flush(&self) -> bool {
        *self == Flush::NotFlush
    }
    #[doc = "Flush FIFO"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == Flush::Flush
    }
}
#[doc = "Field `rxffr` writer - Receiver FIFO reset"]
pub type RxffrW<'a, REG> = crate::BitWriter<'a, REG, Flush>;
impl<'a, REG> RxffrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not flush FIFO"]
    #[inline(always)]
    pub fn not_flush(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::NotFlush)
    }
    #[doc = "Flush FIFO"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::Flush)
    }
}
impl R {
    #[doc = "Bit 0 - Receiver FIFO reset"]
    #[inline(always)]
    pub fn rxffr(&self) -> RxffrR {
        RxffrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver FIFO reset"]
    #[inline(always)]
    #[must_use]
    pub fn rxffr(&mut self) -> RxffrW<RxffrSpec> {
        RxffrW::new(self, 0)
    }
}
#[doc = "Receiver Block FIFO Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxffr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxffr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxffrSpec;
impl crate::RegisterSpec for RxffrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxffr::R`](R) reader structure"]
impl crate::Readable for RxffrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxffr::W`](W) writer structure"]
impl crate::Writable for RxffrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rxffr to value 0"]
impl crate::Resettable for RxffrSpec {
    const RESET_VALUE: u32 = 0;
}
