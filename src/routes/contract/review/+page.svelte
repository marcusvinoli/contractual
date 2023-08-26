<script lang="ts">
  import ContractForm from "$lib/ContractForm.svelte";
  import PessoaFisica from "$lib/PessoaFisica.svelte";
  
  import type { ContractFormParameters } from "$lib/ContractForm.svelte";
  import type { PessoaFisicaType } from "$lib/PessoaFisica.svelte";

  //let params: ContractFormParameters;
  
  let parameters = {
    name: "Novo Contrato",
    steps: [
      "Promitente Vendedor",
      "Promitente Comprador",
      "Objeto",
      "Valores",
      "Revisão",
    ],
    stepIndex: 4,
    stepTip: "Insira os dados do Promitente Comprador.",
    previous: {
      name: "Voltar"
    },
    next: {
      name: "Gerar Contrato"
    },
    onNext: onNextHandler,
    onPrevious: onPreviousHandler,
  }

  let pessoaFisica: PessoaFisicaType = {
	  dadosPessoais: {
		  nome: "",
		  cpf: "",
		  rg: "",
		  emissorRg: ""
	  },
	  dadosConjuge: {
		  nome: "",
		  cpf: "",
		  rg: "",
		  emissorRg: ""
	  },
	  estadoCivil: "",
	  nacionalidade: "",
	  profissao: "",
	  telefone: {
		  celular: "",
		  fixo: ""
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
		  estado: ""
	  }
  }

  function onNextHandler() {
    // console.log("Next button pressed: " + JSON.stringify(pessoaFisica));
    console.log("Next button pressed: ")
    console.log(pessoaFisica);
    window.location.href = "/home"
  }
  
  function onPreviousHandler() {
    console.log("Previous button pressed.");
    window.location.href = "/contract/values"
  }

</script>

<div>
  <ContractForm parameters={parameters}>
    <h1>Objeto do Contrato</h1>
  </ContractForm>
</div>
