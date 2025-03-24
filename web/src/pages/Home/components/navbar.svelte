<script lang="ts">
  import { decodeJwt } from "./jwt";
  import { LogOut } from "@lucide/svelte";
  const payload = decodeJwt();
</script>

<div class="flex flexbox justify-between">
  <p>{payload?.name}</p>
  <a href="/logout">
    <LogOut class="h-5 w-5" />
  </a>
</div>
