#[doc = "Register `INSTR_MEM%s` writer"]
pub struct W(crate::W<INSTR_MEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INSTR_MEM_SPEC>;
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
impl From<crate::W<INSTR_MEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INSTR_MEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTR_MEM0` writer - "]
pub struct INSTR_MEM0_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR_MEM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn instr_mem0(&mut self) -> INSTR_MEM0_W {
        INSTR_MEM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write-only access to instruction memory location %s  

This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [instr_mem](index.html) module"]
pub struct INSTR_MEM_SPEC;
impl crate::RegisterSpec for INSTR_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [instr_mem::W](W) writer structure"]
impl crate::Writable for INSTR_MEM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INSTR_MEM%s to value 0"]
impl crate::Resettable for INSTR_MEM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
