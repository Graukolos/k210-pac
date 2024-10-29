#[doc = "Register `sccb_cfg` reader"]
pub type R = crate::R<SccbCfgSpec>;
#[doc = "Register `sccb_cfg` writer"]
pub type W = crate::W<SccbCfgSpec>;
#[doc = "BYTE_NUM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ByteNum {
    #[doc = "1: BYTE_NUM_2"]
    Num2 = 1,
    #[doc = "2: BYTE_NUM_3"]
    Num3 = 2,
    #[doc = "3: BYTE_NUM_4"]
    Num4 = 3,
}
impl From<ByteNum> for u8 {
    #[inline(always)]
    fn from(variant: ByteNum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ByteNum {
    type Ux = u8;
}
impl crate::IsEnum for ByteNum {}
#[doc = "Field `byte_num` reader - BYTE_NUM"]
pub type ByteNumR = crate::FieldReader<ByteNum>;
impl ByteNumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ByteNum> {
        match self.bits {
            1 => Some(ByteNum::Num2),
            2 => Some(ByteNum::Num3),
            3 => Some(ByteNum::Num4),
            _ => None,
        }
    }
    #[doc = "BYTE_NUM_2"]
    #[inline(always)]
    pub fn is_num2(&self) -> bool {
        *self == ByteNum::Num2
    }
    #[doc = "BYTE_NUM_3"]
    #[inline(always)]
    pub fn is_num3(&self) -> bool {
        *self == ByteNum::Num3
    }
    #[doc = "BYTE_NUM_4"]
    #[inline(always)]
    pub fn is_num4(&self) -> bool {
        *self == ByteNum::Num4
    }
}
#[doc = "Field `byte_num` writer - BYTE_NUM"]
pub type ByteNumW<'a, REG> = crate::FieldWriter<'a, REG, 2, ByteNum>;
impl<'a, REG> ByteNumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BYTE_NUM_2"]
    #[inline(always)]
    pub fn num2(self) -> &'a mut crate::W<REG> {
        self.variant(ByteNum::Num2)
    }
    #[doc = "BYTE_NUM_3"]
    #[inline(always)]
    pub fn num3(self) -> &'a mut crate::W<REG> {
        self.variant(ByteNum::Num3)
    }
    #[doc = "BYTE_NUM_4"]
    #[inline(always)]
    pub fn num4(self) -> &'a mut crate::W<REG> {
        self.variant(ByteNum::Num4)
    }
}
#[doc = "Field `scl_lcnt` reader - SCL_LCNT"]
pub type SclLcntR = crate::FieldReader;
#[doc = "Field `scl_lcnt` writer - SCL_LCNT"]
pub type SclLcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `scl_hcnt` reader - SCL_HCNT"]
pub type SclHcntR = crate::FieldReader;
#[doc = "Field `scl_hcnt` writer - SCL_HCNT"]
pub type SclHcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rdata` reader - RDATA"]
pub type RdataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - BYTE_NUM"]
    #[inline(always)]
    pub fn byte_num(&self) -> ByteNumR {
        ByteNumR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - SCL_LCNT"]
    #[inline(always)]
    pub fn scl_lcnt(&self) -> SclLcntR {
        SclLcntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCL_HCNT"]
    #[inline(always)]
    pub fn scl_hcnt(&self) -> SclHcntR {
        SclHcntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RDATA"]
    #[inline(always)]
    pub fn rdata(&self) -> RdataR {
        RdataR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BYTE_NUM"]
    #[inline(always)]
    #[must_use]
    pub fn byte_num(&mut self) -> ByteNumW<SccbCfgSpec> {
        ByteNumW::new(self, 0)
    }
    #[doc = "Bits 8:15 - SCL_LCNT"]
    #[inline(always)]
    #[must_use]
    pub fn scl_lcnt(&mut self) -> SclLcntW<SccbCfgSpec> {
        SclLcntW::new(self, 8)
    }
    #[doc = "Bits 16:23 - SCL_HCNT"]
    #[inline(always)]
    #[must_use]
    pub fn scl_hcnt(&mut self) -> SclHcntW<SccbCfgSpec> {
        SclHcntW::new(self, 16)
    }
}
#[doc = "SCCB Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sccb_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccb_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccbCfgSpec;
impl crate::RegisterSpec for SccbCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sccb_cfg::R`](R) reader structure"]
impl crate::Readable for SccbCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sccb_cfg::W`](W) writer structure"]
impl crate::Writable for SccbCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sccb_cfg to value 0"]
impl crate::Resettable for SccbCfgSpec {
    const RESET_VALUE: u32 = 0;
}
