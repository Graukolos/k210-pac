#[doc = "Register `data_cmd` reader"]
pub type R = crate::R<DataCmdSpec>;
#[doc = "Register `data_cmd` writer"]
pub type W = crate::W<DataCmdSpec>;
#[doc = "Field `data` reader - Data"]
pub type DataR = crate::FieldReader;
#[doc = "Field `data` writer - Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cmd` reader - CMD"]
pub type CmdR = crate::BitReader;
#[doc = "Field `cmd` writer - CMD"]
pub type CmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - CMD"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<DataCmdSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bit 8 - CMD"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<DataCmdSpec> {
        CmdW::new(self, 8)
    }
}
#[doc = "Data Buffer and Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataCmdSpec;
impl crate::RegisterSpec for DataCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_cmd::R`](R) reader structure"]
impl crate::Readable for DataCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`data_cmd::W`](W) writer structure"]
impl crate::Writable for DataCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets data_cmd to value 0"]
impl crate::Resettable for DataCmdSpec {
    const RESET_VALUE: u32 = 0;
}
