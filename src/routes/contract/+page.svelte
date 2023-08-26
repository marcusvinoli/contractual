<script>
  import ContractForm from "$lib/ContractForm.svelte";
  import PessoaFisica from "$lib/PessoaFisica.svelte";

  let params = {
    name: "Novo Contrato",
    steps: [
      "Promitente Vendedor",
      "Promitente Comprador",
      "Objeto",
      "Valores",
      "Revisão",
    ],
    stepIndex: 0,
    stepTip: "Insira os dados do Promitente Vendedor.",
    previous: {
      name: "Cancelar",
      href: "/home"
    },
    next: {
      name: "Avançar",
      href: "/contract/"
    },
    onNext: () => {
      return onNextHandler()
    },
    onPrevious: () => {

    },
  }

  let value = {
    nome: "",
    cpf: "",
    rg: "",
    estado_civil: "",
    emissor_rg: "",
    nacionalidade: "",
    profissao: "",
    conjuge: {},
    telefone: {
      fixo: "",
      celular: "",
    },
    endereco: {
      cep: "",
      logradouro: "",
      numero: "",
      qd: "",
      lt: "",
      complemeto: "",
      bairro: "",
      cidade: "",
      estado: "",
    },
  }

  function onNextHandler() {
    console.log("Next button pressed: " + JSON.stringify(value))
  }

</script>

<div>
  <ContractForm params={params}>
    <!-- <PessoaFisica bind:value={value}/> -->
  </ContractForm>
</div>