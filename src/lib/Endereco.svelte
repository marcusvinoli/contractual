<script lang="js">
// @ts-nocheck

  import MaskInput from "svelte-input-mask/MaskInput.svelte";
  import EstadoSelector from "./utils/EstadoSelector.svelte";

  import cep from "cep-promise";

  let semNumero = false;
  let cep2find = "";
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

  function loadCepInfo(inputCep = String) {
    console.log(endereco);
    console.log("Running CEP Promise for {}", inputCep);
    cep(inputCep).then(
      console.log
    );
  }

  function test() {
    console.log(cep2find);
  }

  function debounce(func, timeout = 300){
    let timer;
    return (...args) => {
      clearTimeout(timer);
      timer = setTimeout(() => { func.apply(this, args); }, timeout);
    };
  }

  function saveInput(){
    console.log('Saving data');
  }
  const processChange = () => debounce(() => saveInput());

</script>

<div class="w-full">
  <div class="flex w-full">
    <div class="p-1">
      <label for="cep" class="label">CEP</label>
      <MaskInput id="cep" type="text" class="input input-bordered w-50" placeholder="CEP" maskChar="_" mask="00.000-000" bind:value={cep2find} onkeyup="processChange()"/>
      <button class="btn" on:click={test}>Carregar informações do CEP</button>
    </div>
  </div>
  <div class="flex w-full">
    <div class="grow p-1">
      <label for="cep" class="label">Logradouro</label>
      <input id="cep" type="text" placeholder="Logradouro" class="input input-bordered w-full" />
    </div>
    <div class="flex flex-row">
      <div class="align-center p-1">
        <label for="semNumero" class="label">Sem Número</label>
        <input id="semNumero" type="checkbox" bind:checked="{semNumero}" class="checkbox" />
      </div>
      <div class="p-1">
        <label for="numero" class="label">Número</label>
        {#if semNumero}
          <input id="numero" type="text" placeholder="Nº" class="input input-bordered w-24" value="S/N" disabled/>
        {:else}
          <input id="numero" type="text" placeholder="Nº" class="input input-bordered w-24" />
        {/if}
      </div>
    </div>
  </div>
  <div class="flex w-full">
    <div class="p-1 w-28">
      <label for="cep" class="label">Qd.</label>
      <input id="cep" type="text" placeholder="Qd." class="input input-bordered w-full" />
    </div>
    <div class="p-1 w-28">
      <label for="cep" class="label">Lt.</label>
      <input id="cep" type="text" placeholder="Lt." class="input input-bordered w-full" />
    </div>
    <div class="p-1 grow">
      <label for="cep" class="label">Complemento</label>
      <input id="cep" type="text" placeholder="Complemento" class="input input-bordered w-full" />
    </div>
  </div>
  <div class="flex w-full">
  </div>
  <div class="flex w-full">
    <div class="p-1">
      <label for="cep" class="label">Bairro</label>
      <input id="bairro" type="text" placeholder="Bairro" class="input input-bordered w-full" />
    </div>
    <div class="p-1">
      <label for="cep" class="label">Cidade</label>
      <input id="cidade" type="text" placeholder="Cidade" class="input input-bordered w-full" />
    </div>
    <div class="p-1">
      <label for="cep" class="label">Estado</label>
      <EstadoSelector id="estado"/>
    </div>
  </div>
</div>