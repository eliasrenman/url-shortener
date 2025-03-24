<script lang="ts">
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import "./app.css";
  import Home from "./pages/Home/Home.svelte";
  import Login from "./pages/Login/Login.svelte";
  import { queryClient } from "@/api";
  const cookies = document.cookie;
</script>

<QueryClientProvider client={queryClient}>
  <main class="p-4">
    {#if cookies.includes("access_token")}
      <Home />
    {:else}
      <Login />
    {/if}
  </main>
</QueryClientProvider>
