#[doc = "Register `INSTR_MEM%s` writer"]
pub type W = crate::W<INSTR_MEM_SPEC>;
#[doc = "Field `INSTR_MEM0` writer - "]
pub type INSTR_MEM0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn instr_mem0(&mut self) -> INSTR_MEM0_W<INSTR_MEM_SPEC> {
        INSTR_MEM0_W::new(self, 0)
    }
}
#[doc = "Write-only access to instruction memory location %s  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instr_mem::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INSTR_MEM_SPEC;
impl crate::RegisterSpec for INSTR_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`instr_mem::W`](W) writer structure"]
impl crate::Writable for INSTR_MEM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INSTR_MEM%s to value 0"]
impl crate::Resettable for INSTR_MEM_SPEC {
    const RESET_VALUE: u32 = 0;
}
