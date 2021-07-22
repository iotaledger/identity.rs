<script lang="ts">
  import { storeLatestDidDocument } from "../identity-wasm/latest-did-service";

  import { getIntegrationMessageChain } from "../iota-client/integration-message-chain-service";

  import { debugMode } from "../stores/stores";

  let inputValue = "8gS3MfUhSm9qE7TfLi96xV192GMy3cR56MWwMXzosqCQ";
  let debugToggleElement: HTMLInputElement;

  async function handleSearchClick() {
    await getIntegrationMessageChain(inputValue);
    await storeLatestDidDocument(inputValue);
  }

  function debugModeToggle() {
    if (debugToggleElement.checked) {
      debugMode.update((debug) => true);
    } else {
      debugMode.update((debug) => false);
    }
  }
</script>

<div class="d-flex align-items-center justify-content-center">
  <input class="did-input" bind:value={inputValue} type="text" placeholder="did:iota:abc...xyz" />
  <button on:click={handleSearchClick}> Search </button>

  <div class="form-check form-switch  mx-3">
    <input
      class="form-check-input"
      type="checkbox"
      id="debugToggle"
      on:click={debugModeToggle}
      bind:this={debugToggleElement}
    />
    <label class="form-check-label" for="debugToggle">Debug</label>
  </div>
</div>

<style>
  .did-input {
    width: 400px;
  }

  label {
    display: inline;
  }
</style>
