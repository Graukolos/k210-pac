#[doc = "Register `tag_chk` reader"]
pub type R = crate::R<TagChkSpec>;
#[doc = "Register `tag_chk` writer"]
pub type W = crate::W<TagChkSpec>;
#[doc = "Tag check status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TagChk {
    #[doc = "0: Check not finished"]
    Busy = 0,
    #[doc = "1: Check failed"]
    Fail = 1,
    #[doc = "2: Check success"]
    Success = 2,
}
impl From<TagChk> for u8 {
    #[inline(always)]
    fn from(variant: TagChk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TagChk {
    type Ux = u8;
}
impl crate::IsEnum for TagChk {}
#[doc = "Field `tag_chk` reader - Tag check status"]
pub type TagChkR = crate::FieldReader<TagChk>;
impl TagChkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TagChk> {
        match self.bits {
            0 => Some(TagChk::Busy),
            1 => Some(TagChk::Fail),
            2 => Some(TagChk::Success),
            _ => None,
        }
    }
    #[doc = "Check not finished"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TagChk::Busy
    }
    #[doc = "Check failed"]
    #[inline(always)]
    pub fn is_fail(&self) -> bool {
        *self == TagChk::Fail
    }
    #[doc = "Check success"]
    #[inline(always)]
    pub fn is_success(&self) -> bool {
        *self == TagChk::Success
    }
}
#[doc = "Field `tag_chk` writer - Tag check status"]
pub type TagChkW<'a, REG> = crate::FieldWriter<'a, REG, 2, TagChk>;
impl<'a, REG> TagChkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Check not finished"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(TagChk::Busy)
    }
    #[doc = "Check failed"]
    #[inline(always)]
    pub fn fail(self) -> &'a mut crate::W<REG> {
        self.variant(TagChk::Fail)
    }
    #[doc = "Check success"]
    #[inline(always)]
    pub fn success(self) -> &'a mut crate::W<REG> {
        self.variant(TagChk::Success)
    }
}
impl R {
    #[doc = "Bits 0:1 - Tag check status"]
    #[inline(always)]
    pub fn tag_chk(&self) -> TagChkR {
        TagChkR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Tag check status"]
    #[inline(always)]
    #[must_use]
    pub fn tag_chk(&mut self) -> TagChkW<TagChkSpec> {
        TagChkW::new(self, 0)
    }
}
#[doc = "Tag check status\n\nYou can [`read`](crate::Reg::read) this register and get [`tag_chk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tag_chk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TagChkSpec;
impl crate::RegisterSpec for TagChkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tag_chk::R`](R) reader structure"]
impl crate::Readable for TagChkSpec {}
#[doc = "`write(|w| ..)` method takes [`tag_chk::W`](W) writer structure"]
impl crate::Writable for TagChkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tag_chk to value 0"]
impl crate::Resettable for TagChkSpec {
    const RESET_VALUE: u32 = 0;
}
