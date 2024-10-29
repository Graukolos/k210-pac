#[doc = "Register `imr` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `imr` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `rxdam` reader - Mask RX FIFO data avaliable interrupt"]
pub type RxdamR = crate::BitReader;
#[doc = "Field `rxdam` writer - Mask RX FIFO data avaliable interrupt"]
pub type RxdamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxfom` reader - Mask RX FIFO overrun interrupt"]
pub type RxfomR = crate::BitReader;
#[doc = "Field `rxfom` writer - Mask RX FIFO overrun interrupt"]
pub type RxfomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txfem` reader - Mask TX FIFO empty interrupt"]
pub type TxfemR = crate::BitReader;
#[doc = "Field `txfem` writer - Mask TX FIFO empty interrupt"]
pub type TxfemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txfom` reader - Mask TX FIFO overrun interrupt"]
pub type TxfomR = crate::BitReader;
#[doc = "Field `txfom` writer - Mask TX FIFO overrun interrupt"]
pub type TxfomW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask RX FIFO data avaliable interrupt"]
    #[inline(always)]
    pub fn rxdam(&self) -> RxdamR {
        RxdamR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask RX FIFO overrun interrupt"]
    #[inline(always)]
    pub fn rxfom(&self) -> RxfomR {
        RxfomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask TX FIFO empty interrupt"]
    #[inline(always)]
    pub fn txfem(&self) -> TxfemR {
        TxfemR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask TX FIFO overrun interrupt"]
    #[inline(always)]
    pub fn txfom(&self) -> TxfomR {
        TxfomR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask RX FIFO data avaliable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxdam(&mut self) -> RxdamW<ImrSpec> {
        RxdamW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask RX FIFO overrun interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxfom(&mut self) -> RxfomW<ImrSpec> {
        RxfomW::new(self, 1)
    }
    #[doc = "Bit 4 - Mask TX FIFO empty interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txfem(&mut self) -> TxfemW<ImrSpec> {
        TxfemW::new(self, 4)
    }
    #[doc = "Bit 5 - Mask TX FIFO overrun interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txfom(&mut self) -> TxfomW<ImrSpec> {
        TxfomW::new(self, 5)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets imr to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
