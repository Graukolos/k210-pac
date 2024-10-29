#[doc = "Register `power_sel` reader"]
pub type R = crate::R<PowerSelSpec>;
#[doc = "Register `power_sel` writer"]
pub type W = crate::W<PowerSelSpec>;
#[doc = "Field `power_mode_sel0` reader - "]
pub type PowerModeSel0R = crate::BitReader;
#[doc = "Field `power_mode_sel0` writer - "]
pub type PowerModeSel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `power_mode_sel1` reader - "]
pub type PowerModeSel1R = crate::BitReader;
#[doc = "Field `power_mode_sel1` writer - "]
pub type PowerModeSel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `power_mode_sel2` reader - "]
pub type PowerModeSel2R = crate::BitReader;
#[doc = "Field `power_mode_sel2` writer - "]
pub type PowerModeSel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `power_mode_sel3` reader - "]
pub type PowerModeSel3R = crate::BitReader;
#[doc = "Field `power_mode_sel3` writer - "]
pub type PowerModeSel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `power_mode_sel4` reader - "]
pub type PowerModeSel4R = crate::BitReader;
#[doc = "Field `power_mode_sel4` writer - "]
pub type PowerModeSel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `power_mode_sel5` reader - "]
pub type PowerModeSel5R = crate::BitReader;
#[doc = "Field `power_mode_sel5` writer - "]
pub type PowerModeSel5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `power_mode_sel6` reader - "]
pub type PowerModeSel6R = crate::BitReader;
#[doc = "Field `power_mode_sel6` writer - "]
pub type PowerModeSel6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `power_mode_sel7` reader - "]
pub type PowerModeSel7R = crate::BitReader;
#[doc = "Field `power_mode_sel7` writer - "]
pub type PowerModeSel7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn power_mode_sel0(&self) -> PowerModeSel0R {
        PowerModeSel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn power_mode_sel1(&self) -> PowerModeSel1R {
        PowerModeSel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn power_mode_sel2(&self) -> PowerModeSel2R {
        PowerModeSel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn power_mode_sel3(&self) -> PowerModeSel3R {
        PowerModeSel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn power_mode_sel4(&self) -> PowerModeSel4R {
        PowerModeSel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn power_mode_sel5(&self) -> PowerModeSel5R {
        PowerModeSel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn power_mode_sel6(&self) -> PowerModeSel6R {
        PowerModeSel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn power_mode_sel7(&self) -> PowerModeSel7R {
        PowerModeSel7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode_sel0(&mut self) -> PowerModeSel0W<PowerSelSpec> {
        PowerModeSel0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode_sel1(&mut self) -> PowerModeSel1W<PowerSelSpec> {
        PowerModeSel1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode_sel2(&mut self) -> PowerModeSel2W<PowerSelSpec> {
        PowerModeSel2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode_sel3(&mut self) -> PowerModeSel3W<PowerSelSpec> {
        PowerModeSel3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode_sel4(&mut self) -> PowerModeSel4W<PowerSelSpec> {
        PowerModeSel4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode_sel5(&mut self) -> PowerModeSel5W<PowerSelSpec> {
        PowerModeSel5W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode_sel6(&mut self) -> PowerModeSel6W<PowerSelSpec> {
        PowerModeSel6W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode_sel7(&mut self) -> PowerModeSel7W<PowerSelSpec> {
        PowerModeSel7W::new(self, 7)
    }
}
#[doc = "IO Power Mode Select controller\n\nYou can [`read`](crate::Reg::read) this register and get [`power_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerSelSpec;
impl crate::RegisterSpec for PowerSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_sel::R`](R) reader structure"]
impl crate::Readable for PowerSelSpec {}
#[doc = "`write(|w| ..)` method takes [`power_sel::W`](W) writer structure"]
impl crate::Writable for PowerSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets power_sel to value 0"]
impl crate::Resettable for PowerSelSpec {
    const RESET_VALUE: u32 = 0;
}
