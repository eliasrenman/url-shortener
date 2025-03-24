<script lang="ts">
  import { mutateUrl, type UrlUpsertDto } from "@/api/url";
  import { ArrowDown } from "@lucide/svelte";

  const mutation = mutateUrl();
  const handleSubmit = (e: SubmitEvent) => {
    e.preventDefault();
    const target = e.target as HTMLFormElement;
    const data = new FormData(target);

    const formObject = Object.fromEntries(data.entries()) as unknown as {
      url: string;
      destinationUrl: string;
      ttl: string;
    };
    let parsedTtl;
    if (formObject.ttl !== "-1") {
      parsedTtl = new Date(
        new Date().getTime() + parseFloat(formObject.ttl) * 1000,
      );
    }
    $mutation.mutate(
      {
        ...formObject,
        ttl: parsedTtl,
      },
      {
        onSuccess: () => {
          target.reset();
        },
      },
    );
  };
</script>

<form method="POST" on:submit={handleSubmit}>
  <div class="flex mt-4 p-4 bg-gray-800 rounded-lg shadow-lg gap-4 flex-col">
    <div class="flex flex-row gap-2 justify-center items-center">
      <p>{window.location.origin}/</p>
      <input
        class="p-2 bg-gray-900 text-white rounded-lg"
        name="url"
        type="text"
        placeholder="Shortform"
      />
    </div>
    <div class="flex justify-center">
      <ArrowDown />
    </div>
    <div class="flex flex-row gap-2 items-end">
      <input
        class="w-full p-2 bg-gray-900 text-white rounded-lg"
        name="destinationUrl"
        type="url"
        placeholder="Destination URL"
      />

      <div>
        <label for="ttl" class="block text-sm">Expiry</label>
        <select
          name="ttl"
          class="bg-gray-900 text-white rounded-lg p-2 w-full mt-1"
        >
          <option value="-1" selected>Never</option>
          <option value="1800">30 minutes</option>
          <option value="3600">1 hour</option>
          <option value="86400">1 day</option>
          <option value="604800">1 week</option>
          <option value="2592000">1 month</option>
        </select>
      </div>
    </div>
    <button
      class="w-full px-4 py-2 bg-blue-600 text-white rounded-lg shadow hover:bg-blue-500 transition cursor-pointer"
    >
      Shorten!
    </button>
  </div>
</form>
