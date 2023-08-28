use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Telefone {
  pub celular: String,
  pub fixo: String,
}
