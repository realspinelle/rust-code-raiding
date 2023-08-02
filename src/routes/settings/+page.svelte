<script lang="ts">
	import { SlideToggle } from '@skeletonlabs/skeleton';
	import MdKeyboardReturn from 'svelte-icons/md/MdKeyboardReturn.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	let autoPressKey: boolean;
	let randomKeyBind: boolean;
	let keybind: string;
	let loaded = false;
	(async () => {
		keybind = await invoke('get_keybind');
		autoPressKey = await invoke('get_auto_press_key');
		randomKeyBind = await invoke('get_random_key_bind');
	})();
	$: (async () => {
		if (!loaded) return;
		await invoke('set_auto_press_key', { val: autoPressKey });
	})();
	$: (async () => {
		if (!loaded) return;
		await invoke('set_random_key_bind', { val: randomKeyBind });
	})();
	setTimeout(() => {
		loaded = true;
	}, 500);
</script>

<div class="flex flex-col gap-2 items-center center">
	<span class="text-xl">Settings</span>
	<div class="flex text-sm gap-2 items-center">
		Press codes automatically <SlideToggle bind:checked={autoPressKey} name="" size="sm" />
	</div>
	<div class="flex text-sm gap-2 items-center">
		Randomize keybinds (less ban chance) <SlideToggle
			bind:checked={randomKeyBind}
			name=""
			size="sm"
		/>
	</div>
	<div class="flex text-xl gap-2 items-center">keybinds</div>
	<div class="flex text-sm gap-2 items-center">
		auto code : Keypad {keybind}
		{#if keybind != '...'}
			<button
				on:click={async () => {
					await invoke('set_keybind');
					keybind = '...';
				}}
				class="btn-icon-sm rounded variant-filled"><MdKeyboardReturn /></button
			>
		{/if}
	</div>
</div>
