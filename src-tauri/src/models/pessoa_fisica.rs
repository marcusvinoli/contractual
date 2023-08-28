use serde::{Serialize, Deserialize};

use crate::models::dados_pessoais::DadosPessoais;
use crate::models::telefone::Telefone;
use crate::models::endereco::Endereco;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all="camelCase")]
pub struct PessoaFisica {
  pub dados_pessoais: DadosPessoais,
  #[serde(skip_serializing_if="Option::is_none")]
  pub dados_conjuge: Option<DadosPessoais>,
  pub estado_civil: String,
  pub nacionalidade: String,
  pub profissao: String,
  pub telefone: Telefone,
  pub endereco: Endereco,  
}
