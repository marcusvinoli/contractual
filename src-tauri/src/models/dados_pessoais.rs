use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all="camelCase")]
pub struct DadosPessoais {
  pub nome: String,
  pub cpf: String,
  pub rg: String,
  pub emissor_rg: String,
}
