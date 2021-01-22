#[doc = "Reader of register CH5_DBG_CTDREQ"]
pub type R = crate::R<u32, super::CH5_DBG_CTDREQ>;
#[doc = "Reader of field `CH5_DBG_CTDREQ`"]
pub type CH5_DBG_CTDREQ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch5_dbg_ctdreq(&self) -> CH5_DBG_CTDREQ_R {
        CH5_DBG_CTDREQ_R::new((self.bits & 0x3f) as u8)
    }
}
