<script lang='ts'>
import { latestDid } from "../stores/stores";
import { formatJson } from "../utils/jsonUtils";




</script>


<div>
  <pre class="container">
    {formatJson($latestDid)}
  </pre>
  

</div>


<style>
  .container{
    text-align: left;
    background-color: white;
  }
</style>