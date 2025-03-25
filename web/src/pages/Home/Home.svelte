<script lang="ts">
  import Create from "./components/create.svelte";
  import Navbar from "./components/navbar.svelte";
  import { queryUrls } from "@/api/url";
  import Row from "./components/row.svelte";
  const urls = queryUrls();
</script>

<div class="p-4">
  <Navbar />

  <div class="max-w-3xl mx-auto m-4">
    <h1 class="text-2xl font-bold mb-4">URL Shortener</h1>

    <!-- Table -->
    <div class="bg-gray-800 text-white rounded-lg shadow-lg overflow-hidden">
      {#if $urls.isLoading}
        <p>Loading...</p>
      {:else if $urls.isError}
        <p>Error: {$urls.error.message}</p>
      {:else if $urls.isSuccess}
        <table class="w-full text-left">
          <thead class="bg-gray-900">
            <tr>
              <th class="p-3">Shortform</th>
              <th class="p-3">Destination</th>
              <th class="p-3">Expires at</th>
              <th class="p-3">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each $urls.data.data as row}
              <Row {row} />
            {/each}
          </tbody>
        </table>
      {/if}
    </div>

    <Create />
  </div>
</div>
