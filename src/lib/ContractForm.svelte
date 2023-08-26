<script context="module" lang="ts">
  export type ContractFormParameters = {
    name: string,
    steps: Array<string>,
    stepIndex: number,
    stepTip: string,
    previous: any,
    next: any,
    onNext: () => void,
    onPrevious: () => void,
  };

</script>

<script lang="ts">
  export let parameters: ContractFormParameters;

  /* export let parameters: ContractFormParams = {
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
      name: "Avançar",
    },
    onNext: navigate,
    onPrevious: navigate,
  } */
</script>

<div class="p-5 w-full">
  <div class="mb-4 prose">
    <h3>{parameters.name}</h3>
  </div>
  <div class="flex flex-row">
    <div class="w-64">
      <ul class="steps steps-vertical">
        {#each parameters.steps as step, index}
          {#if (index<=parameters.stepIndex)}
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
          <button class="btn" on:click={parameters.onPrevious}>{parameters.previous.name}</button>
        </div>
        <div class="ml-1">
          <button class="btn btn-primary" on:click={parameters.onNext}>{parameters.next.name}</button>
        </div>
      </div>
    </div>
  </div>
</div>
