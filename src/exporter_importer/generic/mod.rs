use crate::vertex::Vertex;
use std::error::Error;

pub(super) fn collect_attributes_str(vertex: &Vertex) -> Result<String, Box<dyn Error>> {
    let attr_str_keys = vertex.get_attr_keys();
    let mut res = String::new();
    let mut cntr = 0;
    for attr_k in attr_str_keys.iter() {
        cntr += 1;
        res.push_str(&attr_k);
        res.push_str(": ");
        res.push_str(&vertex.get_attr(attr_k)?);
        if cntr != attr_str_keys.len() {
            res.push_str(" | ");
        }
    }
    Ok(res)
}
