<script>
  export let id = "";
  export let value = "";
  let estados =[
    "AC",
    "AL",
    "AP",
    "AM",
    "BA",
    "CE",
    "DF",
    "ES",
    "GO",
    "MA",
    "MT",
    "MS",
    "MG",
    "PA",
    "PB",
    "PR",
    "PE",
    "PI",
    "RJ",
    "RN",
    "RS",
    "RO",
    "RR",
    "SC",
    "SP",
    "SE",
    "TO",
  ]
</script>

<select id={id} bind:value={value} class="select select-bordered w-full max-w-xs">
  <option disabled selected>SELECIONE</option>
  {#each estados as estado}
    <option value="{estado}">{estado}</option>
  {/each}
</select>


