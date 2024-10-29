#[doc = "Register `block_ts` reader"]
pub type R = crate::R<BlockTsSpec>;
#[doc = "Register `block_ts` writer"]
pub type W = crate::W<BlockTsSpec>;
#[doc = "Field `block_ts` reader - Block transfer size"]
pub type BlockTsR = crate::FieldReader<u32>;
#[doc = "Field `block_ts` writer - Block transfer size"]
pub type BlockTsW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Block transfer size"]
    #[inline(always)]
    pub fn block_ts(&self) -> BlockTsR {
        BlockTsR::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Block transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn block_ts(&mut self) -> BlockTsW<BlockTsSpec> {
        BlockTsW::new(self, 0)
    }
}
#[doc = "Block Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`block_ts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block_ts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlockTsSpec;
impl crate::RegisterSpec for BlockTsSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`block_ts::R`](R) reader structure"]
impl crate::Readable for BlockTsSpec {}
#[doc = "`write(|w| ..)` method takes [`block_ts::W`](W) writer structure"]
impl crate::Writable for BlockTsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets block_ts to value 0"]
impl crate::Resettable for BlockTsSpec {
    const RESET_VALUE: u64 = 0;
}
