#[doc = "Register `encrypt_sel` reader"]
pub type R = crate::R<EncryptSelSpec>;
#[doc = "Register `encrypt_sel` writer"]
pub type W = crate::W<EncryptSelSpec>;
#[doc = "Select encryption or decryption mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncryptSel {
    #[doc = "0: Sets encryption mode"]
    Encryption = 0,
    #[doc = "1: Sets decryption mode"]
    Decryption = 1,
}
impl From<EncryptSel> for bool {
    #[inline(always)]
    fn from(variant: EncryptSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `encrypt_sel` reader - Select encryption or decryption mode"]
pub type EncryptSelR = crate::BitReader<EncryptSel>;
impl EncryptSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EncryptSel {
        match self.bits {
            false => EncryptSel::Encryption,
            true => EncryptSel::Decryption,
        }
    }
    #[doc = "Sets encryption mode"]
    #[inline(always)]
    pub fn is_encryption(&self) -> bool {
        *self == EncryptSel::Encryption
    }
    #[doc = "Sets decryption mode"]
    #[inline(always)]
    pub fn is_decryption(&self) -> bool {
        *self == EncryptSel::Decryption
    }
}
#[doc = "Field `encrypt_sel` writer - Select encryption or decryption mode"]
pub type EncryptSelW<'a, REG> = crate::BitWriter<'a, REG, EncryptSel>;
impl<'a, REG> EncryptSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sets encryption mode"]
    #[inline(always)]
    pub fn encryption(self) -> &'a mut crate::W<REG> {
        self.variant(EncryptSel::Encryption)
    }
    #[doc = "Sets decryption mode"]
    #[inline(always)]
    pub fn decryption(self) -> &'a mut crate::W<REG> {
        self.variant(EncryptSel::Decryption)
    }
}
impl R {
    #[doc = "Bit 0 - Select encryption or decryption mode"]
    #[inline(always)]
    pub fn encrypt_sel(&self) -> EncryptSelR {
        EncryptSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select encryption or decryption mode"]
    #[inline(always)]
    #[must_use]
    pub fn encrypt_sel(&mut self) -> EncryptSelW<EncryptSelSpec> {
        EncryptSelW::new(self, 0)
    }
}
#[doc = "Encryption or decryption select\n\nYou can [`read`](crate::Reg::read) this register and get [`encrypt_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`encrypt_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EncryptSelSpec;
impl crate::RegisterSpec for EncryptSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`encrypt_sel::R`](R) reader structure"]
impl crate::Readable for EncryptSelSpec {}
#[doc = "`write(|w| ..)` method takes [`encrypt_sel::W`](W) writer structure"]
impl crate::Writable for EncryptSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets encrypt_sel to value 0"]
impl crate::Resettable for EncryptSelSpec {
    const RESET_VALUE: u32 = 0;
}
