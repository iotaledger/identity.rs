<script lang="ts">
import { formatJson } from "../utils/jsonUtils";



  export let messageContent: string;
</script>

<div class="container">
  <pre>
    {formatJson(messageContent)}
  </pre>
</div>

<style>
  .container {
    background-color: white;
    font-size: 1em;
    min-width: 400px;
    height: 600px;
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 10px;
    text-align: left;
    overflow-x: hidden;
  }

  pre {
    overflow: hidden;
    font-size: 0.7em;
  }
</style>
