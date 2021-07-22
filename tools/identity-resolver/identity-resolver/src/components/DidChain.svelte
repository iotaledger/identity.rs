<script>
  import { Converter } from "../client-lib/index-browser";
  import { Messages } from "../stores/stores";
  import DidIntegrationMessage from "./DidIntegrationMessage.svelte";

  function getData(messageContent) {
    console.log(messageContent);
    return Converter.hexToUtf8(messageContent.payload.data);
  }
</script>

<div class="container">
  {#each $Messages as message}
    <DidIntegrationMessage messageContent={getData(message)} />
  {/each}
</div>

<style>
  .container {
    display: flex;
    flex-direction: row;
    width: 100%;
    overflow-x: scroll;
  }
</style>
