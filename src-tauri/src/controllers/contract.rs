use crate::models::contract_data::ContractData;
use crate::models::pessoa_fisica::PessoaFisica;

pub struct Contract {
  pub contract_data: ContractData,
}

impl Contract {
  pub fn new() -> Contract {
    Contract { 
      contract_data: ContractData::default()
     }
  }

  pub fn add_vendor(&mut self, vendedor: PessoaFisica) {
    self.contract_data.promitente_vendedor = vendedor;
  }

  pub fn add_buyer(&mut self, comprador: PessoaFisica) {
    self.contract_data.promitente_comprador = comprador;
  }

  pub fn set_vendor_from_json(&mut self, vendedor: &str) -> std::io::Result<()> {
    let vendedor: PessoaFisica = serde_json::from_str(&vendedor)?;
    self.add_vendor(vendedor);
    return Ok(());
  }

  pub fn set_buyer_from_json(&mut self, comprador: &str) -> std::io::Result<()> {
    let comprador: PessoaFisica = serde_json::from_str(&comprador)?;
    self.add_buyer(comprador);
    return Ok(());
  }

  pub fn get_vendor_as_json(&self) -> String {
    let ret: String = serde_json::to_string(&self.contract_data.promitente_vendedor).unwrap_or_default();
    return ret;
  }

  pub fn get_buyer_as_json(&self) -> String {
    let ret: String = serde_json::to_string(&self.contract_data.promitente_comprador).unwrap_or_default();
    return ret;
  }
}
