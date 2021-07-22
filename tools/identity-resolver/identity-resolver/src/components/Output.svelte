<script lang='ts'>
import { debugMode } from "../stores/stores";
import DidChain from "./DidChain.svelte";
import LatestDidDocument from "./LatestDidDocument.svelte";



</script>

{#if $debugMode}
<DidChain />

{:else}

<LatestDidDocument/>
{/if}