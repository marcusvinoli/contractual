<script lang="ts">
  export let id: string;
  export let value: string | undefined;
  export let mask: string | Array<string>;
  export let customClass: string;
  export let placeholder: string;
  export let masked: boolean = true;
  let lastValue: string | number;
  let display: string | number;
  const transform: (v: string) => string = (v) => v.toLocaleUpperCase();

  const tokens = {
    '#': { pattern: /\d/ },
    X: { pattern: /[0-9a-zA-Z]/ },
    S: { pattern: /[a-zA-Z]/ },
    A: { pattern: /[a-zA-Z]/, transform },
    a: { pattern: /[a-zA-Z]/, transform },
    '!': { escape: true },
  }
  
  function onInput (e: any) {
    // if (e.isTrusted) return; // ignore native event
    value = refresh(e.target.value);
  };

  function refresh (value: string) {
    display = value;
    var newValue = masker(value)
    if (newValue !== lastValue) {
      lastValue = value;
      // $emit('input', newValue) // Check this part
      value = newValue;
    }
    return newValue;
  }

  function masker(value: string) {
    return Array.isArray(mask) ? dynamicMask(maskit, mask)(value, mask, masked) : maskit(value, mask, masked)
  }

  function dynamicMask (maskit: any, masks: Array<string>) {
    masks = masks.sort((a: string, b: string) => a.length - b.length)
    return function (value: string | number, mask: Array<string>, masked = true) {
      var i = 0
      while (i < masks.length) {
        var currentMask = masks[i]
        i++
        var nextMask = masks[i]
        if (! (nextMask && maskit(value, nextMask, true).length > currentMask.length) ) {
          return maskit(value, currentMask, masked)
        }
      }
      return '' // empty masks
    }
  }

  function maskit (value: string, mask: string, masked: boolean) {
    value = value || '';
    mask = mask || '';
    var iMask = 0;
    var iValue = 0;
    var output = '';
    var lenght: number = 0;

    while (iMask < mask.length && iValue < value.length) {
      var cMask = mask[iMask];
      var masker = tokens[cMask as keyof typeof tokens];
      var cValue = value[iValue];

      if (masker && !('escape' in masker)) {
        if (masker.pattern.test(cValue)) {
          if ('trasform' in masker) {
            output += transform(cValue);
          } else {
            output += cValue;
          }
          iMask++
        }
        iValue++
      } else {
        if (masker && ('escape' in masker)) {
          iMask++ // take the next mask char and treat it as char
          cMask = mask[iMask]
        }
        if (masked) output += cMask
        if (cValue === cMask) iValue++ // user typed the same char
        iMask++
      }
    }

    // fix mask that ends with a char: (#)
    var restOutput = ''
    while (iMask < mask.length && masked) {
      var cMask = mask[iMask]
      if (tokens[cMask as keyof typeof tokens]) {
        restOutput = ''
        break
      }
      restOutput += cMask
      iMask++
    }

  return output + restOutput
}

</script>

<input id={id} type="text" placeholder={placeholder} class={customClass} bind:value={value} on:input={onInput}/>
