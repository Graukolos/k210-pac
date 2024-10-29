#[doc = "Register `data_out_flag` reader"]
pub type R = crate::R<DataOutFlagSpec>;
#[doc = "Register `data_out_flag` writer"]
pub type W = crate::W<DataOutFlagSpec>;
#[doc = "Data can be read from out_data when this flag is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataOutFlag {
    #[doc = "0: Data cannot output"]
    CannotOutput = 0,
    #[doc = "1: Data can output"]
    CanOutput = 1,
}
impl From<DataOutFlag> for bool {
    #[inline(always)]
    fn from(variant: DataOutFlag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `data_out_flag` reader - Data can be read from out_data when this flag is set"]
pub type DataOutFlagR = crate::BitReader<DataOutFlag>;
impl DataOutFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataOutFlag {
        match self.bits {
            false => DataOutFlag::CannotOutput,
            true => DataOutFlag::CanOutput,
        }
    }
    #[doc = "Data cannot output"]
    #[inline(always)]
    pub fn is_cannot_output(&self) -> bool {
        *self == DataOutFlag::CannotOutput
    }
    #[doc = "Data can output"]
    #[inline(always)]
    pub fn is_can_output(&self) -> bool {
        *self == DataOutFlag::CanOutput
    }
}
#[doc = "Field `data_out_flag` writer - Data can be read from out_data when this flag is set"]
pub type DataOutFlagW<'a, REG> = crate::BitWriter<'a, REG, DataOutFlag>;
impl<'a, REG> DataOutFlagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data cannot output"]
    #[inline(always)]
    pub fn cannot_output(self) -> &'a mut crate::W<REG> {
        self.variant(DataOutFlag::CannotOutput)
    }
    #[doc = "Data can output"]
    #[inline(always)]
    pub fn can_output(self) -> &'a mut crate::W<REG> {
        self.variant(DataOutFlag::CanOutput)
    }
}
impl R {
    #[doc = "Bit 0 - Data can be read from out_data when this flag is set"]
    #[inline(always)]
    pub fn data_out_flag(&self) -> DataOutFlagR {
        DataOutFlagR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data can be read from out_data when this flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn data_out_flag(&mut self) -> DataOutFlagW<DataOutFlagSpec> {
        DataOutFlagW::new(self, 0)
    }
}
#[doc = "Data can output flag\n\nYou can [`read`](crate::Reg::read) this register and get [`data_out_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_out_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataOutFlagSpec;
impl crate::RegisterSpec for DataOutFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_out_flag::R`](R) reader structure"]
impl crate::Readable for DataOutFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`data_out_flag::W`](W) writer structure"]
impl crate::Writable for DataOutFlagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets data_out_flag to value 0"]
impl crate::Resettable for DataOutFlagSpec {
    const RESET_VALUE: u32 = 0;
}
