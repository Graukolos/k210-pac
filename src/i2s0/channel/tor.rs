#[doc = "Register `tor` reader"]
pub type R = crate::R<TorSpec>;
#[doc = "Field `txcho` reader - Read this bit to clear TX FIFO data overrun interrupt. 0x0 for TX FIFO write valid, 0x1 for TX FIFO write overrun"]
pub type TxchoR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this bit to clear TX FIFO data overrun interrupt. 0x0 for TX FIFO write valid, 0x1 for TX FIFO write overrun"]
    #[inline(always)]
    pub fn txcho(&self) -> TxchoR {
        TxchoR::new((self.bits & 1) != 0)
    }
}
#[doc = "Transmit Overrun Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TorSpec;
impl crate::RegisterSpec for TorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tor::R`](R) reader structure"]
impl crate::Readable for TorSpec {}
#[doc = "`reset()` method sets tor to value 0"]
impl crate::Resettable for TorSpec {
    const RESET_VALUE: u32 = 0;
}
