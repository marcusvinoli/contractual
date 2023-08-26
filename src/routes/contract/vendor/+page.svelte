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
    stepIndex: 0,
    stepTip: "Insira os dados do Promitente Vendedor.",
    previous: {
      name: "Cancelar"
    },
    next: {
      name: "Avançar"
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
    console.log("Next button pressed: ")
    console.log(pessoaFisica);
    window.location.href = "/contract/buyer"
  }
  
  function onPreviousHandler() {
    console.log("Previous button pressed.");
    window.location.href = "/home"
  }

</script>

<div>
  <ContractForm parameters={parameters}>
    <PessoaFisica bind:value={pessoaFisica}/>
  </ContractForm>
</div>
