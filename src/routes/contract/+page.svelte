<script>
  import ContractForm from "$lib/ContractForm.svelte";
  import Endereco from "$lib/Endereco.svelte";
  // export let promitenteVendedor = {
  //   nome: "Paulo Henrique Miguel de Oliveira",
  //   cpf: "000.000.000-00",
  //   rg: "00000/SSP-GO",
  //   estadoCivi: "Solteiro",
  //   endereco: {
  //     cep: "74920060",
  //     logradouro: "Rua Viana Moog",
  //     numero: "S/N",
  //     qd: "100",
  //     lt: "6A",
  //     complemento: "",
  //     bairro: "Cidade Satelite Sao Luiz",
  //     cidade: "Aparecida de Goiânia",
  //     estado: "GO"
  //   }
  // };
</script>

<div>
  <ContractForm>
    <Endereco />
  </ContractForm>
</div>