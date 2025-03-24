<script lang="ts">
  import Icon from "@common/Icon.svelte";
  import { decodeJwt } from "./jwt";

  const payload = decodeJwt();
</script>

<div class="flex flexbox justify-between">
  <p>{payload?.name}</p>
  <a href="/logout">
    <Icon name="logout" class="h-5 w-5" />
  </a>
</div>
