<script lang="ts">
  import { mutateDeleteUrl, type Url } from "@/api/url";
  import { Edit, Trash } from "@lucide/svelte";

  let { row }: { row: Url } = $props();

  const mutation = mutateDeleteUrl();
  function verifyDelete() {
    if (confirm("Are you sure you want to delete this redirect?")) {
      $mutation.mutate(row.url);
    }
  }
</script>

<tr class="border-t border-gray-700 hover:bg-gray-700 transition">
  <td class="p-3 font-mono"
    ><a href={"/" + row.url} aria-label="Redirect url">{row.url}</a></td
  >
  <td class="p-3 truncate"
    ><a href={row.destinationUrl} aria-label="Destination url"
      >{row.destinationUrl}</a
    ></td
  >
  <td class="p-3 truncate"
    >{row.ttl ? new Date(row.ttl).toLocaleTimeString() : "Never"}</td
  >
  <td class="p-3 flex gap-3">
    <button class="text-blue-600 hover:text-blue-500 cursor-pointer transition">
      <Edit size={18} />
    </button>
    <button
      onclick={verifyDelete}
      class="text-red-400 hover:text-red-300 cursor-pointer transition"
    >
      <Trash size={18} />
    </button>
  </td>
</tr>
