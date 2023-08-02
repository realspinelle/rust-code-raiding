<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	let codes: string[] = [];
	(async () => {
		try {
			codes = (JSON.parse(await invoke('get_codes')) as { code: string }[]).map((e) => e.code);
		} catch (e) {
			console.error(e);
		}
	})();
</script>

<div class="flex flex-col gap-2 items-center center">
	<span class="text-xl">Codes list</span>
	<div class="flex flex-col items-center center">
		{#each codes as code}
			<span>{code}</span>
		{/each}
	</div>
</div>
