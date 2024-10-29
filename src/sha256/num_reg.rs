#[doc = "Register `num_reg` reader"]
pub type R = crate::R<NumRegSpec>;
#[doc = "Register `num_reg` writer"]
pub type W = crate::W<NumRegSpec>;
#[doc = "Field `data_cnt` reader - The total amount of data calculated by SHA256 is set by this register, and the smallest unit is 512bit"]
pub type DataCntR = crate::FieldReader<u16>;
#[doc = "Field `data_cnt` writer - The total amount of data calculated by SHA256 is set by this register, and the smallest unit is 512bit"]
pub type DataCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `data_num` reader - Currently calculated block number. 512bit=1block"]
pub type DataNumR = crate::FieldReader<u16>;
#[doc = "Field `data_num` writer - Currently calculated block number. 512bit=1block"]
pub type DataNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The total amount of data calculated by SHA256 is set by this register, and the smallest unit is 512bit"]
    #[inline(always)]
    pub fn data_cnt(&self) -> DataCntR {
        DataCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Currently calculated block number. 512bit=1block"]
    #[inline(always)]
    pub fn data_num(&self) -> DataNumR {
        DataNumR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The total amount of data calculated by SHA256 is set by this register, and the smallest unit is 512bit"]
    #[inline(always)]
    #[must_use]
    pub fn data_cnt(&mut self) -> DataCntW<NumRegSpec> {
        DataCntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Currently calculated block number. 512bit=1block"]
    #[inline(always)]
    #[must_use]
    pub fn data_num(&mut self) -> DataNumW<NumRegSpec> {
        DataNumW::new(self, 16)
    }
}
#[doc = "Counters register\n\nYou can [`read`](crate::Reg::read) this register and get [`num_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`num_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NumRegSpec;
impl crate::RegisterSpec for NumRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`num_reg::R`](R) reader structure"]
impl crate::Readable for NumRegSpec {}
#[doc = "`write(|w| ..)` method takes [`num_reg::W`](W) writer structure"]
impl crate::Writable for NumRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets num_reg to value 0"]
impl crate::Resettable for NumRegSpec {
    const RESET_VALUE: u32 = 0;
}
