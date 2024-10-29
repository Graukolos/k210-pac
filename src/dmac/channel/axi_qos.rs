#[doc = "Register `axi_qos` reader"]
pub type R = crate::R<AxiQosSpec>;
#[doc = "Register `axi_qos` writer"]
pub type W = crate::W<AxiQosSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "AXI QOS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_qos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_qos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiQosSpec;
impl crate::RegisterSpec for AxiQosSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`axi_qos::R`](R) reader structure"]
impl crate::Readable for AxiQosSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_qos::W`](W) writer structure"]
impl crate::Writable for AxiQosSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets axi_qos to value 0"]
impl crate::Resettable for AxiQosSpec {
    const RESET_VALUE: u64 = 0;
}
