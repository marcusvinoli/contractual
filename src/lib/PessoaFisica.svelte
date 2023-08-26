<script context="module" lang="ts">
  import type { EnderecoType } from "./Endereco.svelte";
    
  export type DadosPessoais = {
    nome: string,
    cpf: string,
    rg: string,
    emissorRg: string,
  }

  export type Telefone = {
    celular: string,
    fixo: string,
  }

  export type PessoaFisicaType = {
    dadosPessoais: DadosPessoais,
    dadosConjuge: DadosPessoais,
    estadoCivil: string,
    nacionalidade: string,
    profissao: string,
    telefone: Telefone,
    endereco: EnderecoType,
  }

</script>

<script lang="ts">
	import Endereco from "./Endereco.svelte";
  import MaskedInput from "./utils/MaskedInput.svelte";
  import EstadoCivilSelector from "./utils/EstadoCivilSelector.svelte";
  
  export let value: PessoaFisicaType;
  
  // Dummies values to prevent Server Errors.
  let dummy: DadosPessoais;
  let dummyPhone: string;

</script>

<div>
  <div class="flex flex-col w-full">
    <div class="prose">
      <h2>Dados Pessoais</h2> 
    </div>
    <div class="grid card place-items-center p-2 mb-3">
      <div class="flex w-full">
        <div class="p-1 grow">
          <label for="nome" class="label">Nome</label>
          <input id="nome" type="text" placeholder="José da Silva" class="input input-bordered w-full" bind:value={value.dadosPessoais.nome}/>
        </div>
        <div class="p-1 w-64">
          <label for="cpf" class="label">CPF</label>
          <MaskedInput id="cpf" placeholder="00.000.000-00" customClass="input input-bordered w-full p-1" mask="###.###.###-##" bind:value={value.dadosPessoais.cpf}/>
        </div>
      </div>
      <div class="flex w-full">
        <div class="p-1 grow">
          <label for="rg" class="label">RG</label>
          <input id="rg" type="text" placeholder="00000" class="input input-bordered w-full" bind:value={value.dadosPessoais.rg}/>
        </div>
        <div class="p-1 grow">
          <label for="emiss" class="label">Órgão Emissor</label>
          <input id="emiss" type="text" placeholder="SSP-GO" class="input input-bordered w-full grow" bind:value={value.dadosPessoais.emissorRg}/>
        </div>
        <div class="p-1 w-64">
          <label for="estdo_civil" class="label">Estado Civil</label>
          <EstadoCivilSelector id="estado_civil" bind:value={value.estadoCivil} />
        </div>
      </div>
      <div class="flex w-full">
        <div class="p-1 grow">
          <label for="nacionalidade" class="label">Nacionalidade</label>
          <input id="nacionalidade" type="text" placeholder="Brasileiro" class="input input-bordered w-full" bind:value={value.nacionalidade}/>
        </div>
        <div class="p-1 grow">
          <label for="profissao" class="label">Profissão</label>
          <input id="profissao" type="text" placeholder="Corretor" class="input input-bordered w-full grow" bind:value={value.profissao}/>
        </div>
      </div>
    </div>
    {#if (value.estadoCivil === String("Casado"))}
      <div class="prose">
        <h2>Dados Pessoais - Conjuge</h2> 
      </div>
      <div class="grid card place-items-center p-2 mb-3">
        <div class="flex w-full">
          <div class="p-1 grow">
            <label for="nome" class="label">Nome</label>
            <input id="nome" type="text" placeholder="Maria da Silva" class="input input-bordered w-full" bind:value={value.dadosConjuge.nome}/>
          </div>
          <div class="p-1 w-64">
            <label for="cpf" class="label">CPF</label>
            <MaskedInput id="cpf" placeholder="00.000.000-00" customClass="input input-bordered w-full p-1" mask="###.###.###-##" bind:value={value.dadosConjuge.cpf}/>
          </div>
        </div>
        <div class="flex w-full">
          <div class="p-1 grow">
            <label for="rg" class="label">RG</label>
            <input id="rg" type="text" placeholder="00000" class="input input-bordered w-full" bind:value={value.dadosConjuge.rg}/>
          </div>
          <div class="p-1 grow">
            <label for="emiss" class="label">Órgão Emissor</label>
            <input id="emiss" type="text" placeholder="SSP-GO" class="input input-bordered w-full grow" bind:value={value.dadosConjuge.emissorRg}/>
          </div>
        </div>
      </div>
    {/if}
    <div class="prose">
      <h2>Contato</h2> 
    </div>
    <div class="grid card place-items-center p-2 mb-3">
      <div class="flex w-full">
        <div class="p-1 grow">
          <label for="telefone1" class="label">Celular</label>
          <MaskedInput id="telefone1" placeholder="(00) 00000-0000" customClass="input input-bordered w-full p-1" mask="{'(##) #####-####'}" bind:value={value.telefone.celular}/>
        </div>
        <div class="p-1 grow">
          <label for="telefone2" class="label">Fixo/Celular</label>
          <MaskedInput id="telefone2" placeholder="(00) 0000-0000" customClass="input input-bordered w-full p-1" mask="{['(##) ####-####', '(##) #####-####']}" bind:value={value.telefone.fixo}/>
        </div>
      </div>
    </div>
    <div class="prose">
      <h2>Endereço</h2> 
    </div>
    <div class="grid card place-items-center p-2 mb-3">
      <Endereco value={value.endereco}/>
    </div>
  </div>
</div>
