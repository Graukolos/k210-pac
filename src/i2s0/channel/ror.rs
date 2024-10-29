#[doc = "Register `ror` reader"]
pub type R = crate::R<RorSpec>;
#[doc = "Field `rxcho` reader - Read this bit to clear RX FIFO data overrun interrupt. 0x0 for RX FIFO write valid, 0x1 for RX FIFO write overrun"]
pub type RxchoR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this bit to clear RX FIFO data overrun interrupt. 0x0 for RX FIFO write valid, 0x1 for RX FIFO write overrun"]
    #[inline(always)]
    pub fn rxcho(&self) -> RxchoR {
        RxchoR::new((self.bits & 1) != 0)
    }
}
#[doc = "Receive Overrun Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ror::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RorSpec;
impl crate::RegisterSpec for RorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ror::R`](R) reader structure"]
impl crate::Readable for RorSpec {}
#[doc = "`reset()` method sets ror to value 0"]
impl crate::Resettable for RorSpec {
    const RESET_VALUE: u32 = 0;
}
