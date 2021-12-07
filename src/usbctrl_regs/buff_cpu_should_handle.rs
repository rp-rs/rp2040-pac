#[doc = "Register `BUFF_CPU_SHOULD_HANDLE` reader"]
pub struct R(crate::R<BUFF_CPU_SHOULD_HANDLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFF_CPU_SHOULD_HANDLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFF_CPU_SHOULD_HANDLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFF_CPU_SHOULD_HANDLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EP15_OUT` reader - "]
pub struct EP15_OUT_R(crate::FieldReader<bool, bool>);
impl EP15_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP15_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP15_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP15_IN` reader - "]
pub struct EP15_IN_R(crate::FieldReader<bool, bool>);
impl EP15_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP15_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP15_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP14_OUT` reader - "]
pub struct EP14_OUT_R(crate::FieldReader<bool, bool>);
impl EP14_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP14_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP14_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP14_IN` reader - "]
pub struct EP14_IN_R(crate::FieldReader<bool, bool>);
impl EP14_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP14_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP14_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP13_OUT` reader - "]
pub struct EP13_OUT_R(crate::FieldReader<bool, bool>);
impl EP13_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP13_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP13_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP13_IN` reader - "]
pub struct EP13_IN_R(crate::FieldReader<bool, bool>);
impl EP13_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP13_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP13_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP12_OUT` reader - "]
pub struct EP12_OUT_R(crate::FieldReader<bool, bool>);
impl EP12_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP12_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP12_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP12_IN` reader - "]
pub struct EP12_IN_R(crate::FieldReader<bool, bool>);
impl EP12_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP12_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP12_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP11_OUT` reader - "]
pub struct EP11_OUT_R(crate::FieldReader<bool, bool>);
impl EP11_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP11_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP11_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP11_IN` reader - "]
pub struct EP11_IN_R(crate::FieldReader<bool, bool>);
impl EP11_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP11_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP11_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP10_OUT` reader - "]
pub struct EP10_OUT_R(crate::FieldReader<bool, bool>);
impl EP10_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP10_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP10_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP10_IN` reader - "]
pub struct EP10_IN_R(crate::FieldReader<bool, bool>);
impl EP10_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP10_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP10_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP9_OUT` reader - "]
pub struct EP9_OUT_R(crate::FieldReader<bool, bool>);
impl EP9_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP9_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP9_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP9_IN` reader - "]
pub struct EP9_IN_R(crate::FieldReader<bool, bool>);
impl EP9_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP9_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP9_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP8_OUT` reader - "]
pub struct EP8_OUT_R(crate::FieldReader<bool, bool>);
impl EP8_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP8_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP8_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP8_IN` reader - "]
pub struct EP8_IN_R(crate::FieldReader<bool, bool>);
impl EP8_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP8_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP8_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP7_OUT` reader - "]
pub struct EP7_OUT_R(crate::FieldReader<bool, bool>);
impl EP7_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP7_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP7_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP7_IN` reader - "]
pub struct EP7_IN_R(crate::FieldReader<bool, bool>);
impl EP7_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP7_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP7_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP6_OUT` reader - "]
pub struct EP6_OUT_R(crate::FieldReader<bool, bool>);
impl EP6_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP6_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP6_IN` reader - "]
pub struct EP6_IN_R(crate::FieldReader<bool, bool>);
impl EP6_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP6_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP5_OUT` reader - "]
pub struct EP5_OUT_R(crate::FieldReader<bool, bool>);
impl EP5_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP5_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP5_IN` reader - "]
pub struct EP5_IN_R(crate::FieldReader<bool, bool>);
impl EP5_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP5_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP4_OUT` reader - "]
pub struct EP4_OUT_R(crate::FieldReader<bool, bool>);
impl EP4_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP4_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP4_IN` reader - "]
pub struct EP4_IN_R(crate::FieldReader<bool, bool>);
impl EP4_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP4_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP3_OUT` reader - "]
pub struct EP3_OUT_R(crate::FieldReader<bool, bool>);
impl EP3_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP3_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP3_IN` reader - "]
pub struct EP3_IN_R(crate::FieldReader<bool, bool>);
impl EP3_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP3_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2_OUT` reader - "]
pub struct EP2_OUT_R(crate::FieldReader<bool, bool>);
impl EP2_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP2_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2_IN` reader - "]
pub struct EP2_IN_R(crate::FieldReader<bool, bool>);
impl EP2_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP2_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1_OUT` reader - "]
pub struct EP1_OUT_R(crate::FieldReader<bool, bool>);
impl EP1_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP1_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1_IN` reader - "]
pub struct EP1_IN_R(crate::FieldReader<bool, bool>);
impl EP1_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP1_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0_OUT` reader - "]
pub struct EP0_OUT_R(crate::FieldReader<bool, bool>);
impl EP0_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0_IN` reader - "]
pub struct EP0_IN_R(crate::FieldReader<bool, bool>);
impl EP0_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ep15_out(&self) -> EP15_OUT_R {
        EP15_OUT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ep15_in(&self) -> EP15_IN_R {
        EP15_IN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ep14_out(&self) -> EP14_OUT_R {
        EP14_OUT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ep14_in(&self) -> EP14_IN_R {
        EP14_IN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ep13_out(&self) -> EP13_OUT_R {
        EP13_OUT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ep13_in(&self) -> EP13_IN_R {
        EP13_IN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ep12_out(&self) -> EP12_OUT_R {
        EP12_OUT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ep12_in(&self) -> EP12_IN_R {
        EP12_IN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ep11_out(&self) -> EP11_OUT_R {
        EP11_OUT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ep11_in(&self) -> EP11_IN_R {
        EP11_IN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ep10_out(&self) -> EP10_OUT_R {
        EP10_OUT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ep10_in(&self) -> EP10_IN_R {
        EP10_IN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ep9_out(&self) -> EP9_OUT_R {
        EP9_OUT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ep9_in(&self) -> EP9_IN_R {
        EP9_IN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ep8_out(&self) -> EP8_OUT_R {
        EP8_OUT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ep8_in(&self) -> EP8_IN_R {
        EP8_IN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep7_out(&self) -> EP7_OUT_R {
        EP7_OUT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep7_in(&self) -> EP7_IN_R {
        EP7_IN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ep6_out(&self) -> EP6_OUT_R {
        EP6_OUT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ep6_in(&self) -> EP6_IN_R {
        EP6_IN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ep5_out(&self) -> EP5_OUT_R {
        EP5_OUT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ep5_in(&self) -> EP5_IN_R {
        EP5_IN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ep4_out(&self) -> EP4_OUT_R {
        EP4_OUT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ep4_in(&self) -> EP4_IN_R {
        EP4_IN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ep3_out(&self) -> EP3_OUT_R {
        EP3_OUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ep3_in(&self) -> EP3_IN_R {
        EP3_IN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ep2_out(&self) -> EP2_OUT_R {
        EP2_OUT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ep2_in(&self) -> EP2_IN_R {
        EP2_IN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ep1_out(&self) -> EP1_OUT_R {
        EP1_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ep1_in(&self) -> EP1_IN_R {
        EP1_IN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ep0_out(&self) -> EP0_OUT_R {
        EP0_OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ep0_in(&self) -> EP0_IN_R {
        EP0_IN_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [buff_cpu_should_handle](index.html) module"]
pub struct BUFF_CPU_SHOULD_HANDLE_SPEC;
impl crate::RegisterSpec for BUFF_CPU_SHOULD_HANDLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buff_cpu_should_handle::R](R) reader structure"]
impl crate::Readable for BUFF_CPU_SHOULD_HANDLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUFF_CPU_SHOULD_HANDLE to value 0"]
impl crate::Resettable for BUFF_CPU_SHOULD_HANDLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
