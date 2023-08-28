use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Endereco {
  pub cep: String,
  pub logradouro: String,
  pub numero: String,
  pub qd: String,
  pub lt: String,
  pub complemento: String,
  pub bairro: String,
  pub cidade: String,
  pub estado: String
}
