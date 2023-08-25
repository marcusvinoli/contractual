<script>
  export let value = "";
  export let id="";
  let estadosCivis =[
    "Solteiro",
    "Casado",
    "Divorciado",
    "Separado",
    "Viúvo",
  ]
</script>

<select id={id} bind:value={value} class="select select-bordered w-full max-w-xs">
  <option disabled selected>Selecione</option>
  {#each estadosCivis as estadoCivil}
    <option value={estadoCivil}>{estadoCivil}</option>
  {/each}
</select>
