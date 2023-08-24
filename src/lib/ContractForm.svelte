<script>
  export let params = {
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
      name: "Cancelar",
      href: "/home"
    },
    next: {
      name: "Avançar",
      href: "/contract/"
    },
  }
</script>

<div class="p-5 w-full">
  <div class="mb-4 prose">
    <h3>{params.name}</h3>
  </div>
  <div class="flex flex-row">
    <div class="w-64">
      <ul class="steps steps-vertical">
        {#each params.steps as step, index}
          {#if index<=params.stepIndex}
            <li class="step step-primary">{step}</li>
          {:else}
            <li class="step">{step}</li>
          {/if}
        {/each}
      </ul>
    </div>
    <div class="grow flex flex-col">
      <div class="grow">
        <slot /> 
      </div>
      <div class="flex justify-end">
        <div class="ml-1">
          <a class="btn" href={params.previous.href}>{params.previous.name}</a>
        </div>
        <div class="ml-1">
          <a class="btn btn-primary" href={params.next.href}>{params.next.name}</a>
        </div>
      </div>
    </div>
  </div>
</div>
