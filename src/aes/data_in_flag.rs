#[doc = "Register `data_in_flag` reader"]
pub type R = crate::R<DataInFlagSpec>;
#[doc = "Register `data_in_flag` writer"]
pub type W = crate::W<DataInFlagSpec>;
#[doc = "Data can be written to text_data or aad_data when this flag is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanInput {
    #[doc = "0: Cannot input"]
    CannotInput = 0,
    #[doc = "1: Can input"]
    CanInput = 1,
}
impl From<CanInput> for bool {
    #[inline(always)]
    fn from(variant: CanInput) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `data_in_flag` reader - Data can be written to text_data or aad_data when this flag is set"]
pub type DataInFlagR = crate::BitReader<CanInput>;
impl DataInFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CanInput {
        match self.bits {
            false => CanInput::CannotInput,
            true => CanInput::CanInput,
        }
    }
    #[doc = "Cannot input"]
    #[inline(always)]
    pub fn is_cannot_input(&self) -> bool {
        *self == CanInput::CannotInput
    }
    #[doc = "Can input"]
    #[inline(always)]
    pub fn is_can_input(&self) -> bool {
        *self == CanInput::CanInput
    }
}
#[doc = "Field `data_in_flag` writer - Data can be written to text_data or aad_data when this flag is set"]
pub type DataInFlagW<'a, REG> = crate::BitWriter<'a, REG, CanInput>;
impl<'a, REG> DataInFlagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cannot input"]
    #[inline(always)]
    pub fn cannot_input(self) -> &'a mut crate::W<REG> {
        self.variant(CanInput::CannotInput)
    }
    #[doc = "Can input"]
    #[inline(always)]
    pub fn can_input(self) -> &'a mut crate::W<REG> {
        self.variant(CanInput::CanInput)
    }
}
impl R {
    #[doc = "Bit 0 - Data can be written to text_data or aad_data when this flag is set"]
    #[inline(always)]
    pub fn data_in_flag(&self) -> DataInFlagR {
        DataInFlagR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data can be written to text_data or aad_data when this flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn data_in_flag(&mut self) -> DataInFlagW<DataInFlagSpec> {
        DataInFlagW::new(self, 0)
    }
}
#[doc = "Data can input flag\n\nYou can [`read`](crate::Reg::read) this register and get [`data_in_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_in_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataInFlagSpec;
impl crate::RegisterSpec for DataInFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_in_flag::R`](R) reader structure"]
impl crate::Readable for DataInFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`data_in_flag::W`](W) writer structure"]
impl crate::Writable for DataInFlagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets data_in_flag to value 0"]
impl crate::Resettable for DataInFlagSpec {
    const RESET_VALUE: u32 = 0;
}
