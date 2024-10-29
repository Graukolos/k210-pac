#[doc = "Register `sync_level` reader"]
pub type R = crate::R<SyncLevelSpec>;
#[doc = "Register `sync_level` writer"]
pub type W = crate::W<SyncLevelSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Sync level registers\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_level::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_level::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncLevelSpec;
impl crate::RegisterSpec for SyncLevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_level::R`](R) reader structure"]
impl crate::Readable for SyncLevelSpec {}
#[doc = "`write(|w| ..)` method takes [`sync_level::W`](W) writer structure"]
impl crate::Writable for SyncLevelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sync_level to value 0"]
impl crate::Resettable for SyncLevelSpec {
    const RESET_VALUE: u32 = 0;
}
