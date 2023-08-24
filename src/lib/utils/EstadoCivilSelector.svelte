<script>
  export let id="";
  let estadosCivis =[
    "Solteiro",
    "Casado",
    "Divorciado",
    "Separado",
    "Viúvo",
  ]
</script>

<select id="{id}" class="select select-bordered w-full max-w-xs">
  <option disabled selected>Selecione</option>
  {#each estadosCivis as estadoCivil}
    <option>{estadoCivil}</option>
  {/each}
</select>
