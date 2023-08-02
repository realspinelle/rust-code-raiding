<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { appWindow } from '@tauri-apps/api/window';

	let autoPressKey: boolean;
	let randomKeyBind: boolean;
	let prev_code: string;
	let next_code: string;
	let cur_code: string;
	let keybind: number;
	(async () => {
		autoPressKey = await invoke('get_auto_press_key');
		randomKeyBind = await invoke('get_random_key_bind');
		keybind = await invoke('get_keybind');
		prev_code = await invoke('get_prev_code');
		next_code = await invoke('get_next_code');
		cur_code = await invoke('get_cur_code');
		appWindow.listen('keybind_changed', (e) => {
			keybind = e.payload as number;
		});
		appWindow.listen('update_cur_code', (e) => {
			cur_code = e.payload as string;
		});
		appWindow.listen('update_prev_code', (e) => {
			prev_code = e.payload as string;
		});
		appWindow.listen('update_next_code', (e) => {
			next_code = e.payload as string;
		});
	})();
</script>

<div class="flex flex-col items-center">
	<span class="text-7xl mt-10">{cur_code}</span>
	<span class="text-lg mt-1">previous code : {prev_code}</span>
	<span class="text-lg mt-1">next code : {next_code}</span>
	{#if randomKeyBind && autoPressKey}
		<span class="text-lg mt-10">keybind : Keypad {keybind}</span>
	{/if}
</div>
