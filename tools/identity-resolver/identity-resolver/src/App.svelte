<script lang="ts">
  import DidInput from "./components/DidInput.svelte";
  import Output from "./components/Output.svelte";
</script>

<main>
  <DidInput />
  <Output />
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
