<script lang="ts">
  import { invoke } from '@tauri-apps/api'
  import { onMount } from 'svelte';

  import ContractForm from "$lib/ContractForm.svelte";
  import PessoaFisica from "$lib/PessoaFisica.svelte";
  
  import type { ContractFormParameters } from "$lib/ContractForm.svelte";
  import type { PessoaFisicaType } from "$lib/PessoaFisica.svelte";
	import type { parse } from 'svelte/compiler';

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
		  complemento: "",
		  bairro: "",
		  cidade: "",
		  estado: ""
	  }
  }

  onMount(() => {
    console.log("Page mounted.");
    invoke('read_vendor').then((data: any) => {
      pessoaFisica = JSON.parse(data);
      console.log(data);
    }
    )
  })

  function onNextHandler() {
    console.log("Next button pressed: ")
    console.log(pessoaFisica);
    invoke('write_vendor', {data: JSON.stringify(pessoaFisica)}).then(() => {
      window.location.href = "/contract/buyer"
    }
    )
  }
  
  function onPreviousHandler() {
    console.log("Previous button pressed.");
    window.location.href = "/"
  }

</script>

<div class="w-full overflow-hidden">
  <ContractForm parameters={parameters}>
    <PessoaFisica bind:value={pessoaFisica}/>
  </ContractForm>
</div>
