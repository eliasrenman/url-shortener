<script lang="ts">
  import Create from "./components/create.svelte";
  import Navbar from "./components/navbar.svelte";
  import { queryUrls } from "@/api/url";
  import Row from "./components/row.svelte";
  const urls = queryUrls();
</script>

<Navbar />

<div class="h-screen flex items-center justify-center flex-col">
  <table>
    <thead>
      <tr>
        <th>Shortform</th>
        <th>Destination</th>
        <th>Time to live</th>
        <th>Action</th>
      </tr>
    </thead>
    {#if $urls.isLoading}
      <tr>
        <td><p>Loading...</p></td>
      </tr>
    {:else if $urls.isError}
      <tr>
        <td><p>Error: {$urls.error.message}</p></td>
      </tr>
    {:else if $urls.isSuccess}
      {#each $urls.data.data as row}
        <Row {row} />
      {/each}
    {/if}
  </table>
  <Create />
</div>
