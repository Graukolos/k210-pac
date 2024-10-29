#[doc = "Register `isr` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `rxda` reader - Status of receiver data avaliable interrupt"]
pub type RxdaR = crate::BitReader;
#[doc = "Field `rxfo` reader - Status of data overrun interrupt for RX channel"]
pub type RxfoR = crate::BitReader;
#[doc = "Field `txfe` reader - Status of transmit empty triger interrupt"]
pub type TxfeR = crate::BitReader;
#[doc = "Field `txfo` reader - Status of data overrun interrupt for the TX channel"]
pub type TxfoR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status of receiver data avaliable interrupt"]
    #[inline(always)]
    pub fn rxda(&self) -> RxdaR {
        RxdaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of data overrun interrupt for RX channel"]
    #[inline(always)]
    pub fn rxfo(&self) -> RxfoR {
        RxfoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of transmit empty triger interrupt"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of data overrun interrupt for the TX channel"]
    #[inline(always)]
    pub fn txfo(&self) -> TxfoR {
        TxfoR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets isr to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
