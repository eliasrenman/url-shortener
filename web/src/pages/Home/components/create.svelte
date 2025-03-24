<script lang="ts">
  import { mutateUrl, type UrlUpsertDto } from "@/api/url";
  import Icon from "@/common/Icon.svelte";

  const mutation = mutateUrl();
  const handleSubmit = (e: SubmitEvent) => {
    e.preventDefault();

    const data = new FormData(e.target as HTMLFormElement);

    const formObject = Object.fromEntries(
      data.entries(),
    ) as unknown as UrlUpsertDto;

    console.log("Form Data:", formObject);
    $mutation.mutate(formObject);
  };
</script>

<form method="POST" on:submit={handleSubmit}>
  <input name="url" type="text" class="border-b-1" />
  <input name="destinationUrl" type="url" class="border-b-1" />
  <button type="submit"><Icon name="add" /></button>
</form>
