<script>
  export let id = "";
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

<select id="{id}" class="select select-bordered w-full max-w-xs">
  <option disabled selected>Selecione</option>
  {#each estados as estado}
    <option>{estado}</option>
  {/each}
</select>


