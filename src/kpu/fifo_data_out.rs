#[doc = "Register `fifo_data_out` reader"]
pub type R = crate::R<FifoDataOutSpec>;
#[doc = "Register `fifo_data_out` writer"]
pub type W = crate::W<FifoDataOutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO data output\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_data_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_data_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoDataOutSpec;
impl crate::RegisterSpec for FifoDataOutSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`fifo_data_out::R`](R) reader structure"]
impl crate::Readable for FifoDataOutSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_data_out::W`](W) writer structure"]
impl crate::Writable for FifoDataOutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets fifo_data_out to value 0"]
impl crate::Resettable for FifoDataOutSpec {
    const RESET_VALUE: u64 = 0;
}
