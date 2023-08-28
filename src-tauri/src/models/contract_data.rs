use serde::{Serialize, Deserialize};

use super::pessoa_fisica::PessoaFisica;

#[derive(Serialize, Deserialize, Default)]
pub struct ContractData {
  pub promitente_vendedor: PessoaFisica,
  pub promitente_comprador: PessoaFisica,
}
