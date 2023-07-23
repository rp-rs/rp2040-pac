#[doc = "Register `BUFF_STATUS` reader"]
pub struct R(crate::R<BUFF_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFF_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFF_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFF_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUFF_STATUS` writer"]
pub struct W(crate::W<BUFF_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFF_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BUFF_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFF_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP0_IN` reader - "]
pub type EP0_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP0_IN` writer - "]
pub type EP0_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP0_OUT` reader - "]
pub type EP0_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP0_OUT` writer - "]
pub type EP0_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP1_IN` reader - "]
pub type EP1_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP1_IN` writer - "]
pub type EP1_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP1_OUT` reader - "]
pub type EP1_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP1_OUT` writer - "]
pub type EP1_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP2_IN` reader - "]
pub type EP2_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP2_IN` writer - "]
pub type EP2_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP2_OUT` reader - "]
pub type EP2_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP2_OUT` writer - "]
pub type EP2_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP3_IN` reader - "]
pub type EP3_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP3_IN` writer - "]
pub type EP3_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP3_OUT` reader - "]
pub type EP3_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP3_OUT` writer - "]
pub type EP3_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP4_IN` reader - "]
pub type EP4_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP4_IN` writer - "]
pub type EP4_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP4_OUT` reader - "]
pub type EP4_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP4_OUT` writer - "]
pub type EP4_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP5_IN` reader - "]
pub type EP5_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP5_IN` writer - "]
pub type EP5_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP5_OUT` reader - "]
pub type EP5_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP5_OUT` writer - "]
pub type EP5_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP6_IN` reader - "]
pub type EP6_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP6_IN` writer - "]
pub type EP6_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP6_OUT` reader - "]
pub type EP6_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP6_OUT` writer - "]
pub type EP6_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP7_IN` reader - "]
pub type EP7_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP7_IN` writer - "]
pub type EP7_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP7_OUT` reader - "]
pub type EP7_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP7_OUT` writer - "]
pub type EP7_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP8_IN` reader - "]
pub type EP8_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP8_IN` writer - "]
pub type EP8_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP8_OUT` reader - "]
pub type EP8_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP8_OUT` writer - "]
pub type EP8_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP9_IN` reader - "]
pub type EP9_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP9_IN` writer - "]
pub type EP9_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP9_OUT` reader - "]
pub type EP9_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP9_OUT` writer - "]
pub type EP9_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP10_IN` reader - "]
pub type EP10_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP10_IN` writer - "]
pub type EP10_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP10_OUT` reader - "]
pub type EP10_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP10_OUT` writer - "]
pub type EP10_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP11_IN` reader - "]
pub type EP11_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP11_IN` writer - "]
pub type EP11_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP11_OUT` reader - "]
pub type EP11_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP11_OUT` writer - "]
pub type EP11_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP12_IN` reader - "]
pub type EP12_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP12_IN` writer - "]
pub type EP12_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP12_OUT` reader - "]
pub type EP12_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP12_OUT` writer - "]
pub type EP12_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP13_IN` reader - "]
pub type EP13_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP13_IN` writer - "]
pub type EP13_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP13_OUT` reader - "]
pub type EP13_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP13_OUT` writer - "]
pub type EP13_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP14_IN` reader - "]
pub type EP14_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP14_IN` writer - "]
pub type EP14_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP14_OUT` reader - "]
pub type EP14_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP14_OUT` writer - "]
pub type EP14_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP15_IN` reader - "]
pub type EP15_IN_R = crate::BitReader<bool>;
#[doc = "Field `EP15_IN` writer - "]
pub type EP15_IN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
#[doc = "Field `EP15_OUT` reader - "]
pub type EP15_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP15_OUT` writer - "]
pub type EP15_OUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, BUFF_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ep0_in(&self) -> EP0_IN_R {
        EP0_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ep0_out(&self) -> EP0_OUT_R {
        EP0_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ep1_in(&self) -> EP1_IN_R {
        EP1_IN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ep1_out(&self) -> EP1_OUT_R {
        EP1_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ep2_in(&self) -> EP2_IN_R {
        EP2_IN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ep2_out(&self) -> EP2_OUT_R {
        EP2_OUT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ep3_in(&self) -> EP3_IN_R {
        EP3_IN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ep3_out(&self) -> EP3_OUT_R {
        EP3_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ep4_in(&self) -> EP4_IN_R {
        EP4_IN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ep4_out(&self) -> EP4_OUT_R {
        EP4_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ep5_in(&self) -> EP5_IN_R {
        EP5_IN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ep5_out(&self) -> EP5_OUT_R {
        EP5_OUT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ep6_in(&self) -> EP6_IN_R {
        EP6_IN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ep6_out(&self) -> EP6_OUT_R {
        EP6_OUT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep7_in(&self) -> EP7_IN_R {
        EP7_IN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep7_out(&self) -> EP7_OUT_R {
        EP7_OUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ep8_in(&self) -> EP8_IN_R {
        EP8_IN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ep8_out(&self) -> EP8_OUT_R {
        EP8_OUT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ep9_in(&self) -> EP9_IN_R {
        EP9_IN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ep9_out(&self) -> EP9_OUT_R {
        EP9_OUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ep10_in(&self) -> EP10_IN_R {
        EP10_IN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ep10_out(&self) -> EP10_OUT_R {
        EP10_OUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ep11_in(&self) -> EP11_IN_R {
        EP11_IN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ep11_out(&self) -> EP11_OUT_R {
        EP11_OUT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ep12_in(&self) -> EP12_IN_R {
        EP12_IN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ep12_out(&self) -> EP12_OUT_R {
        EP12_OUT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ep13_in(&self) -> EP13_IN_R {
        EP13_IN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ep13_out(&self) -> EP13_OUT_R {
        EP13_OUT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ep14_in(&self) -> EP14_IN_R {
        EP14_IN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ep14_out(&self) -> EP14_OUT_R {
        EP14_OUT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ep15_in(&self) -> EP15_IN_R {
        EP15_IN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ep15_out(&self) -> EP15_OUT_R {
        EP15_OUT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_in(&mut self) -> EP0_IN_W<0> {
        EP0_IN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_out(&mut self) -> EP0_OUT_W<1> {
        EP0_OUT_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ep1_in(&mut self) -> EP1_IN_W<2> {
        EP1_IN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ep1_out(&mut self) -> EP1_OUT_W<3> {
        EP1_OUT_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ep2_in(&mut self) -> EP2_IN_W<4> {
        EP2_IN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ep2_out(&mut self) -> EP2_OUT_W<5> {
        EP2_OUT_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_in(&mut self) -> EP3_IN_W<6> {
        EP3_IN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_out(&mut self) -> EP3_OUT_W<7> {
        EP3_OUT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ep4_in(&mut self) -> EP4_IN_W<8> {
        EP4_IN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ep4_out(&mut self) -> EP4_OUT_W<9> {
        EP4_OUT_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ep5_in(&mut self) -> EP5_IN_W<10> {
        EP5_IN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ep5_out(&mut self) -> EP5_OUT_W<11> {
        EP5_OUT_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_in(&mut self) -> EP6_IN_W<12> {
        EP6_IN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_out(&mut self) -> EP6_OUT_W<13> {
        EP6_OUT_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_in(&mut self) -> EP7_IN_W<14> {
        EP7_IN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_out(&mut self) -> EP7_OUT_W<15> {
        EP7_OUT_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ep8_in(&mut self) -> EP8_IN_W<16> {
        EP8_IN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ep8_out(&mut self) -> EP8_OUT_W<17> {
        EP8_OUT_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn ep9_in(&mut self) -> EP9_IN_W<18> {
        EP9_IN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ep9_out(&mut self) -> EP9_OUT_W<19> {
        EP9_OUT_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ep10_in(&mut self) -> EP10_IN_W<20> {
        EP10_IN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ep10_out(&mut self) -> EP10_OUT_W<21> {
        EP10_OUT_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn ep11_in(&mut self) -> EP11_IN_W<22> {
        EP11_IN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ep11_out(&mut self) -> EP11_OUT_W<23> {
        EP11_OUT_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn ep12_in(&mut self) -> EP12_IN_W<24> {
        EP12_IN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn ep12_out(&mut self) -> EP12_OUT_W<25> {
        EP12_OUT_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn ep13_in(&mut self) -> EP13_IN_W<26> {
        EP13_IN_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn ep13_out(&mut self) -> EP13_OUT_W<27> {
        EP13_OUT_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn ep14_in(&mut self) -> EP14_IN_W<28> {
        EP14_IN_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn ep14_out(&mut self) -> EP14_OUT_W<29> {
        EP14_OUT_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn ep15_in(&mut self) -> EP15_IN_W<30> {
        EP15_IN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn ep15_out(&mut self) -> EP15_OUT_W<31> {
        EP15_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [buff_status](index.html) module"]
pub struct BUFF_STATUS_SPEC;
impl crate::RegisterSpec for BUFF_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buff_status::R](R) reader structure"]
impl crate::Readable for BUFF_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buff_status::W](W) writer structure"]
impl crate::Writable for BUFF_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets BUFF_STATUS to value 0"]
impl crate::Resettable for BUFF_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
