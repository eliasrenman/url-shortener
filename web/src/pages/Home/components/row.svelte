<script lang="ts">
  import { mutateDeleteUrl, type Url } from "@/api/url";
  import Icon from "@/common/icon.svelte";

  let { row }: { row: Url } = $props();
  const mutation = mutateDeleteUrl();
  function verifyDelete() {
    if (confirm("Are you sure you want to delete this redirect?")) {
      $mutation.mutate(row.url);
    }
  }
</script>

<tr class="border-b-1">
  <td><a href={"/" + row.url} aria-label="Redirect url">{row.url}</a></td>
  <td
    ><a href={row.destinationUrl} aria-label="Destination url"
      >{row.destinationUrl}</a
    ></td
  >
  <td></td>
  <td class="flex flex-row justify-around"
    ><Icon name="edit" />
    <button onclick={verifyDelete}><Icon name="delete" /></button>
  </td>
</tr>
