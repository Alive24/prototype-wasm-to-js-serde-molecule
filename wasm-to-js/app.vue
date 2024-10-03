<template>
  <div>
    <NuxtRouteAnnouncer />
    <NuxtWelcome />
  </div>
</template>

<script>
import { main, proto_to_js } from '../pkg/wasm_to_js_serde_molecule'

console.log("Loaded");
console.log(proto_to_js());
console.log(main());

</script>
