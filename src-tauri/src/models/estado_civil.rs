use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub enum EstadoCivil {
  #[default]
  Solteiro,
  Casado,
  Divorciado,
  Separador,
  #[serde(rename="Viúvo")]
  Viuvo,
}
