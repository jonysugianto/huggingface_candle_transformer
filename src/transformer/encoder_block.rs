use super::{
    add_norm::AddNorm, mh_selfattention::MHSelfAttention, positionwise_ffn::PositionWiseFFN,
};
use candle_core::{Result, Tensor};
use candle_nn::VarBuilder;

#[derive(Debug)]
pub struct EncoderBlock {
    pub self_attn: MHSelfAttention,
    pub addnorm1: AddNorm,
    pub feed_forward: PositionWiseFFN,
    pub addnorm2: AddNorm,
}

impl EncoderBlock {
    pub fn new(
        vb: &VarBuilder,
        d_model: usize,
        num_heads: usize,
        ff_d_hidden: usize,
    ) -> Result<Self> {
        let sa = MHSelfAttention::new(vb, d_model, num_heads, false, false)?;
        let ff = PositionWiseFFN::new(vb, d_model, ff_d_hidden)?;
        let n1 = AddNorm::new(vb, d_model)?;
        let n2 = AddNorm::new(vb, d_model)?;
        let ret = EncoderBlock {
            addnorm1: n1,
            addnorm2: n2,
            self_attn: sa,
            feed_forward: ff,
        };

        return Ok(ret);
    }

    pub fn forward(&self, xs: &Tensor, mask: Option<&Tensor>) -> Result<Tensor> {
        let attn_out = self.self_attn.forward(xs, mask)?;
        let out = self.addnorm1.forward(xs, &attn_out)?;
        let ff_out = self.feed_forward.forward(&out)?;
        let out = self.addnorm2.forward(&out, &ff_out);
        return out;
    }
}
