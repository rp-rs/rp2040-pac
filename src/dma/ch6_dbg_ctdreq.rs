#[doc = "Reader of register CH6_DBG_CTDREQ"]
pub type R = crate::R<u32, super::CH6_DBG_CTDREQ>;
#[doc = "Reader of field `CH6_DBG_CTDREQ`"]
pub type CH6_DBG_CTDREQ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch6_dbg_ctdreq(&self) -> CH6_DBG_CTDREQ_R {
        CH6_DBG_CTDREQ_R::new((self.bits & 0x3f) as u8)
    }
}
