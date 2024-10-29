#[doc = "Register `ier` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `ier` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `ien` reader - I2S Enable"]
pub type IenR = crate::BitReader;
#[doc = "Field `ien` writer - I2S Enable"]
pub type IenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2S Enable"]
    #[inline(always)]
    pub fn ien(&self) -> IenR {
        IenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IenW<IerSpec> {
        IenW::new(self, 0)
    }
}
#[doc = "Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ier to value 0"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0;
}
