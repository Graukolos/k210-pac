#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    key: [Key; 4],
    encrypt_sel: EncryptSel,
    mode_ctl: ModeCtl,
    iv: [Iv; 4],
    endian: Endian,
    finish: Finish,
    dma_sel: DmaSel,
    aad_num: AadNum,
    _reserved8: [u8; 0x04],
    pc_num: PcNum,
    text_data: TextData,
    aad_data: AadData,
    tag_chk: TagChk,
    data_in_flag: DataInFlag,
    gcm_in_tag: [GcmInTag; 4],
    out_data: OutData,
    en: En,
    data_out_flag: DataOutFlag,
    tag_in_flag: TagInFlag,
    tag_clear: TagClear,
    gcm_out_tag: [GcmOutTag; 4],
    key_ext: [KeyExt; 4],
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - 1st-4th word of key"]
    #[inline(always)]
    pub const fn key(&self, n: usize) -> &Key {
        &self.key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - 1st-4th word of key"]
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &Key> {
        self.key.iter()
    }
    #[doc = "0x10 - Encryption or decryption select"]
    #[inline(always)]
    pub const fn encrypt_sel(&self) -> &EncryptSel {
        &self.encrypt_sel
    }
    #[doc = "0x14 - AES mode register"]
    #[inline(always)]
    pub const fn mode_ctl(&self) -> &ModeCtl {
        &self.mode_ctl
    }
    #[doc = "0x18..0x28 - Initialisation Vector (96 bit for GCM, 128 bit for CBC)"]
    #[inline(always)]
    pub const fn iv(&self, n: usize) -> &Iv {
        &self.iv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x28 - Initialisation Vector (96 bit for GCM, 128 bit for CBC)"]
    #[inline(always)]
    pub fn iv_iter(&self) -> impl Iterator<Item = &Iv> {
        self.iv.iter()
    }
    #[doc = "0x28 - Endian control"]
    #[inline(always)]
    pub const fn endian(&self) -> &Endian {
        &self.endian
    }
    #[doc = "0x2c - Finished status"]
    #[inline(always)]
    pub const fn finish(&self) -> &Finish {
        &self.finish
    }
    #[doc = "0x30 - DMA select"]
    #[inline(always)]
    pub const fn dma_sel(&self) -> &DmaSel {
        &self.dma_sel
    }
    #[doc = "0x34 - GCM additional authenticated data count in bytes, minus one"]
    #[inline(always)]
    pub const fn aad_num(&self) -> &AadNum {
        &self.aad_num
    }
    #[doc = "0x3c - Plaintext/ciphertext input data count in bytes, minus one"]
    #[inline(always)]
    pub const fn pc_num(&self) -> &PcNum {
        &self.pc_num
    }
    #[doc = "0x40 - Plaintext/ciphertext input data"]
    #[inline(always)]
    pub const fn text_data(&self) -> &TextData {
        &self.text_data
    }
    #[doc = "0x44 - Additional authenticated data"]
    #[inline(always)]
    pub const fn aad_data(&self) -> &AadData {
        &self.aad_data
    }
    #[doc = "0x48 - Tag check status"]
    #[inline(always)]
    pub const fn tag_chk(&self) -> &TagChk {
        &self.tag_chk
    }
    #[doc = "0x4c - Data can input flag"]
    #[inline(always)]
    pub const fn data_in_flag(&self) -> &DataInFlag {
        &self.data_in_flag
    }
    #[doc = "0x50..0x60 - GCM input tag for comparison with the calculated tag"]
    #[inline(always)]
    pub const fn gcm_in_tag(&self, n: usize) -> &GcmInTag {
        &self.gcm_in_tag[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x60 - GCM input tag for comparison with the calculated tag"]
    #[inline(always)]
    pub fn gcm_in_tag_iter(&self) -> impl Iterator<Item = &GcmInTag> {
        self.gcm_in_tag.iter()
    }
    #[doc = "0x60 - Plaintext/ciphertext output data"]
    #[inline(always)]
    pub const fn out_data(&self) -> &OutData {
        &self.out_data
    }
    #[doc = "0x64 - AES module enable"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x68 - Data can output flag"]
    #[inline(always)]
    pub const fn data_out_flag(&self) -> &DataOutFlag {
        &self.data_out_flag
    }
    #[doc = "0x6c - Can input tag (when using GCM)"]
    #[inline(always)]
    pub const fn tag_in_flag(&self) -> &TagInFlag {
        &self.tag_in_flag
    }
    #[doc = "0x70 - Tag clear (a write to this register clears the tag_chk status)"]
    #[inline(always)]
    pub const fn tag_clear(&self) -> &TagClear {
        &self.tag_clear
    }
    #[doc = "0x74..0x84 - Computed GCM output tag"]
    #[inline(always)]
    pub const fn gcm_out_tag(&self, n: usize) -> &GcmOutTag {
        &self.gcm_out_tag[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x74..0x84 - Computed GCM output tag"]
    #[inline(always)]
    pub fn gcm_out_tag_iter(&self) -> impl Iterator<Item = &GcmOutTag> {
        self.gcm_out_tag.iter()
    }
    #[doc = "0x84..0x94 - 5th-8th word of key"]
    #[inline(always)]
    pub const fn key_ext(&self, n: usize) -> &KeyExt {
        &self.key_ext[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x84..0x94 - 5th-8th word of key"]
    #[inline(always)]
    pub fn key_ext_iter(&self) -> impl Iterator<Item = &KeyExt> {
        self.key_ext.iter()
    }
}
#[doc = "key (rw) register accessor: 1st-4th word of key\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`]
module"]
#[doc(alias = "key")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "1st-4th word of key"]
pub mod key;
#[doc = "encrypt_sel (rw) register accessor: Encryption or decryption select\n\nYou can [`read`](crate::Reg::read) this register and get [`encrypt_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`encrypt_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@encrypt_sel`]
module"]
#[doc(alias = "encrypt_sel")]
pub type EncryptSel = crate::Reg<encrypt_sel::EncryptSelSpec>;
#[doc = "Encryption or decryption select"]
pub mod encrypt_sel;
#[doc = "mode_ctl (rw) register accessor: AES mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode_ctl`]
module"]
#[doc(alias = "mode_ctl")]
pub type ModeCtl = crate::Reg<mode_ctl::ModeCtlSpec>;
#[doc = "AES mode register"]
pub mod mode_ctl;
#[doc = "iv (rw) register accessor: Initialisation Vector (96 bit for GCM, 128 bit for CBC)\n\nYou can [`read`](crate::Reg::read) this register and get [`iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv`]
module"]
#[doc(alias = "iv")]
pub type Iv = crate::Reg<iv::IvSpec>;
#[doc = "Initialisation Vector (96 bit for GCM, 128 bit for CBC)"]
pub mod iv;
#[doc = "endian (rw) register accessor: Endian control\n\nYou can [`read`](crate::Reg::read) this register and get [`endian::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endian::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endian`]
module"]
#[doc(alias = "endian")]
pub type Endian = crate::Reg<endian::EndianSpec>;
#[doc = "Endian control"]
pub mod endian;
#[doc = "finish (rw) register accessor: Finished status\n\nYou can [`read`](crate::Reg::read) this register and get [`finish::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`finish::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@finish`]
module"]
#[doc(alias = "finish")]
pub type Finish = crate::Reg<finish::FinishSpec>;
#[doc = "Finished status"]
pub mod finish;
#[doc = "dma_sel (rw) register accessor: DMA select\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_sel`]
module"]
#[doc(alias = "dma_sel")]
pub type DmaSel = crate::Reg<dma_sel::DmaSelSpec>;
#[doc = "DMA select"]
pub mod dma_sel;
#[doc = "aad_num (rw) register accessor: GCM additional authenticated data count in bytes, minus one\n\nYou can [`read`](crate::Reg::read) this register and get [`aad_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aad_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aad_num`]
module"]
#[doc(alias = "aad_num")]
pub type AadNum = crate::Reg<aad_num::AadNumSpec>;
#[doc = "GCM additional authenticated data count in bytes, minus one"]
pub mod aad_num;
#[doc = "pc_num (rw) register accessor: Plaintext/ciphertext input data count in bytes, minus one\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_num`]
module"]
#[doc(alias = "pc_num")]
pub type PcNum = crate::Reg<pc_num::PcNumSpec>;
#[doc = "Plaintext/ciphertext input data count in bytes, minus one"]
pub mod pc_num;
#[doc = "text_data (rw) register accessor: Plaintext/ciphertext input data\n\nYou can [`read`](crate::Reg::read) this register and get [`text_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`text_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_data`]
module"]
#[doc(alias = "text_data")]
pub type TextData = crate::Reg<text_data::TextDataSpec>;
#[doc = "Plaintext/ciphertext input data"]
pub mod text_data;
#[doc = "aad_data (rw) register accessor: Additional authenticated data\n\nYou can [`read`](crate::Reg::read) this register and get [`aad_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aad_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aad_data`]
module"]
#[doc(alias = "aad_data")]
pub type AadData = crate::Reg<aad_data::AadDataSpec>;
#[doc = "Additional authenticated data"]
pub mod aad_data;
#[doc = "tag_chk (rw) register accessor: Tag check status\n\nYou can [`read`](crate::Reg::read) this register and get [`tag_chk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tag_chk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tag_chk`]
module"]
#[doc(alias = "tag_chk")]
pub type TagChk = crate::Reg<tag_chk::TagChkSpec>;
#[doc = "Tag check status"]
pub mod tag_chk;
#[doc = "data_in_flag (rw) register accessor: Data can input flag\n\nYou can [`read`](crate::Reg::read) this register and get [`data_in_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_in_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_in_flag`]
module"]
#[doc(alias = "data_in_flag")]
pub type DataInFlag = crate::Reg<data_in_flag::DataInFlagSpec>;
#[doc = "Data can input flag"]
pub mod data_in_flag;
#[doc = "gcm_in_tag (rw) register accessor: GCM input tag for comparison with the calculated tag\n\nYou can [`read`](crate::Reg::read) this register and get [`gcm_in_tag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcm_in_tag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcm_in_tag`]
module"]
#[doc(alias = "gcm_in_tag")]
pub type GcmInTag = crate::Reg<gcm_in_tag::GcmInTagSpec>;
#[doc = "GCM input tag for comparison with the calculated tag"]
pub mod gcm_in_tag;
#[doc = "out_data (rw) register accessor: Plaintext/ciphertext output data\n\nYou can [`read`](crate::Reg::read) this register and get [`out_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_data`]
module"]
#[doc(alias = "out_data")]
pub type OutData = crate::Reg<out_data::OutDataSpec>;
#[doc = "Plaintext/ciphertext output data"]
pub mod out_data;
#[doc = "en (rw) register accessor: AES module enable\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`]
module"]
#[doc(alias = "en")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "AES module enable"]
pub mod en;
#[doc = "data_out_flag (rw) register accessor: Data can output flag\n\nYou can [`read`](crate::Reg::read) this register and get [`data_out_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_out_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_out_flag`]
module"]
#[doc(alias = "data_out_flag")]
pub type DataOutFlag = crate::Reg<data_out_flag::DataOutFlagSpec>;
#[doc = "Data can output flag"]
pub mod data_out_flag;
#[doc = "tag_in_flag (rw) register accessor: Can input tag (when using GCM)\n\nYou can [`read`](crate::Reg::read) this register and get [`tag_in_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tag_in_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tag_in_flag`]
module"]
#[doc(alias = "tag_in_flag")]
pub type TagInFlag = crate::Reg<tag_in_flag::TagInFlagSpec>;
#[doc = "Can input tag (when using GCM)"]
pub mod tag_in_flag;
#[doc = "tag_clear (rw) register accessor: Tag clear (a write to this register clears the tag_chk status)\n\nYou can [`read`](crate::Reg::read) this register and get [`tag_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tag_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tag_clear`]
module"]
#[doc(alias = "tag_clear")]
pub type TagClear = crate::Reg<tag_clear::TagClearSpec>;
#[doc = "Tag clear (a write to this register clears the tag_chk status)"]
pub mod tag_clear;
#[doc = "gcm_out_tag (rw) register accessor: Computed GCM output tag\n\nYou can [`read`](crate::Reg::read) this register and get [`gcm_out_tag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcm_out_tag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcm_out_tag`]
module"]
#[doc(alias = "gcm_out_tag")]
pub type GcmOutTag = crate::Reg<gcm_out_tag::GcmOutTagSpec>;
#[doc = "Computed GCM output tag"]
pub mod gcm_out_tag;
#[doc = "key_ext (rw) register accessor: 5th-8th word of key\n\nYou can [`read`](crate::Reg::read) this register and get [`key_ext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_ext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_ext`]
module"]
#[doc(alias = "key_ext")]
pub type KeyExt = crate::Reg<key_ext::KeyExtSpec>;
#[doc = "5th-8th word of key"]
pub mod key_ext;
