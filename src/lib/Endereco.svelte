<script lang="ts">
// @ts-nocheck

  import MaskedInput from "./utils/MaskedInput.svelte";
  import EstadoSelector from "./utils/EstadoSelector.svelte";

  import cep from "cep-promise";

  let loading = '';
  let errorMessage = ''
  let semNumero = false;

  export let endereco = {
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

  function startLoading() {
    loading = 'loading';
  }

  function stopLoading() {
    loading = '';
  }

  function loadResults(data) {
    endereco.logradouro = data.street;
    endereco.bairro = data.neighborhood;
    endereco.cidade = data.city;
    endereco.estado = data.state;
    stopLoading();
  }
  
  function loadError(e) {
    console.log(e);
    stopLoading();
  }

  function loadCepInfo(inputCep = String) {
    startLoading();
    var cepData = endereco.cep.replace(/[^\d]/g, '');
    console.log("Searching for" + cepData);
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
        <MaskedInput id="cep" type="text" bind:value={endereco.cep} customClass="input input-bordered w-50 p-1" mask="##.###-###"/>
        <button class="btn ml-2" on:click={loadCepInfo}>
          <span class="{loading} loading-spinner text-primary"></span>
          Pesquisar
          <span class=""></span>
        </button>
      </div>
    </div>
    <div class="grow p-1">
      <label for="cep" class="label">Logradouro</label>
      <input id="cep" type="text" placeholder="Logradouro" class="input input-bordered w-full" bind:value={endereco.logradouro}/>
    </div>
    <div class="flex flex-row">
      <div class="content-center p-1">
        <label for="semNumero" class="label">Sem Número</label>
        <div class="p-2" style="text-align: center;">
          <input id="semNumero" type="checkbox" bind:checked="{semNumero}" class="checkbox checkbox-lg" />
        </div>
      </div>
      <div class="p-1">
        <label for="numero" class="label">Número</label>
        {#if semNumero}
          <input id="numero" type="text" placeholder="Nº" class="input input-bordered w-24" value="S/N" disabled/>
        {:else}
          <input id="numero" type="text" placeholder="Nº" class="input input-bordered w-24" bind:value={endereco.numero}/>
        {/if}
      </div>
    </div>
  </div>
  <div class="flex w-full">
    <div class="p-1 w-28">
      <label for="qd" class="label">Qd.</label>
      <input id="qd" type="text" placeholder="Qd." class="input input-bordered w-full" />
    </div>
    <div class="p-1 w-28">
      <label for="lt" class="label">Lt.</label>
      <input id="lt" type="text" placeholder="Lt." class="input input-bordered w-full" />
    </div>
    <div class="p-1 grow">
      <label for="complemento" class="label">Complemento</label>
      <input id="complemento" type="text" placeholder="Complemento" class="input input-bordered w-full" />
    </div>
  </div>
  <div class="flex w-full">
  </div>
  <div class="flex w-full">
    <div class="p-1 grow">
      <label for="bairro" class="label">Bairro</label>
      <input id="bairro" type="text" placeholder="Bairro" class="input input-bordered w-full" bind:value={endereco.bairro}/>
    </div>
    <div class="p-1 grow">
      <label for="cidade" class="label">Cidade</label>
      <input id="cidade" type="text" placeholder="Cidade" class="input input-bordered w-full" bind:value={endereco.cidade}/>
    </div>
    <div class="p-1">
      <label for="estado" class="label">Estado</label>
      <EstadoSelector id="estado" bind:value={endereco.estado}/>
    </div>
  </div>
</div>