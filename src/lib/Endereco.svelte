<script context="module" lang="ts">
  export type EnderecoType = {
    cep: string,
    logradouro: string,
    numero: string,
    qd: string,
    lt: string,
    complemento: string,
    bairro: string,
    cidade: string,
    estado: string,
  }
  
</script>

<script lang="ts">
  import MaskedInput from "./utils/MaskedInput.svelte";
  import EstadoSelector from "./utils/EstadoSelector.svelte";

  import cep from "cep-promise";
	import { onMount } from "svelte";

  let loading = '';
  let semNumero = false;
  // let errorMessage = '' // TODO: Implement error messaging

  export let value: EnderecoType;

  onMount(() => {
    if(value.numero === "S/N") {
      semNumero = true;
    }
  })

  function updateNumero(e: any) {
    console.log(e)
    value.numero = semNumero? "S/N" : "";
    console.log(semNumero);
    console.log(value.numero);
  }

  function startLoading() {
    loading = 'loading';
  }

  function stopLoading() {
    loading = '';
  }

  function loadResults(data: any) {
    value.logradouro = data.street;
    value.bairro = data.neighborhood;
    value.cidade = data.city;
    value.estado = data.state;
    stopLoading();
  }
  
  function loadError(e: any) {
    console.log(e);
    stopLoading();
  }

  function loadCepInfo() {
    startLoading();
    var cepData = value.cep.replace(/[^\d]/g, '');
    cep(cepData)
    .then(
      loadResults
    )
    .catch(
      loadError
    )
  }
</script>

<div class="w-full">
  <div class="flex w-full">
    <div class="p-1">
      <label for="cep" class="label">CEP</label>
      <div class="flex flex-row w-full">
        <MaskedInput id="cep"  placeholder="00000-000" customClass="input input-bordered w-50 p-1" mask="#####-###" bind:value={value.cep}/>
        <button class="btn ml-2" on:click={loadCepInfo}>
          <span class="{loading} loading-spinner text-primary"></span>
          Pesquisar
          <span class=""></span>
        </button>
      </div>
    </div>
    <div class="grow p-1">
      <label for="logradouro" class="label">Logradouro</label>
      <input id="logradouro" type="text" placeholder="Rua José da Silva" class="input input-bordered w-full" bind:value={value.logradouro}/>
    </div>
    <div class="flex flex-row">
      <div class="content-center p-1">
        <label for="sem_numero" class="label">Sem Número</label>
        <div class="p-2" style="text-align: center;">
          <input id="sem_numero" type="checkbox" class="checkbox checkbox-lg" bind:checked={semNumero}/>
        </div>
      </div>
      <div class="p-1">
        <label for="numero" class="label">Número</label>
        {#if semNumero}
        <input id="numero" type="text" placeholder="S/N" class="input input-bordered w-24" bind:value={value.numero} disabled/>
        {:else}
          <input id="numero" type="text" placeholder="Nº" class="input input-bordered w-24" bind:value={value.numero}/>
        {/if}
      </div>
    </div>
  </div>
  <div class="flex w-full">
    <div class="p-1 w-28">
      <label for="qd" class="label">Qd.</label>
      <input id="qd" type="text" placeholder="00" class="input input-bordered w-full" bind:value={value.qd}/>
    </div>
    <div class="p-1 w-28">
      <label for="lt" class="label">Lt.</label>
      <input id="lt" type="text" placeholder="00" class="input input-bordered w-full" bind:value={value.lt}/>
    </div>
    <div class="p-1 grow">
      <label for="complemento" class="label">Complemento</label>
      <input id="complemento" type="text" placeholder="Apt. 00" class="input input-bordered w-full" bind:value={value.complemento}/>
    </div>
  </div>
  <div class="flex w-full">
  </div>
  <div class="flex w-full">
    <div class="p-1 grow">
      <label for="bairro" class="label">Bairro</label>
      <input id="bairro" type="text" placeholder="Bairro" class="input input-bordered w-full" bind:value={value.bairro}/>
    </div>
    <div class="p-1 grow">
      <label for="cidade" class="label">Cidade</label>
      <input id="cidade" type="text" placeholder="Cidade" class="input input-bordered w-full" bind:value={value.cidade}/>
    </div>
    <div class="p-1">
      <label for="estado" class="label">Estado</label>
      <EstadoSelector id="estado" bind:value={value.estado}/>
    </div>
  </div>
</div>
