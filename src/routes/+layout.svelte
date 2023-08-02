<script>
	import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	import '../app.postcss';
	import MdHome from 'svelte-icons/md/MdHome.svelte'
	import MdSettings from 'svelte-icons/md/MdSettings.svelte';
	import MdFormatListNumbered from 'svelte-icons/md/MdFormatListNumbered.svelte';
</script>

<div style="width: 100vw; height: 100vh; position: relative; overflow: hidden">
	<div
		class="bg-slate-950 rounded overflow-y-auto"
		style="height: calc(100vh - 70px); width: calc(100% - 20px); position: absolute; top: 10px; left: 10px;"
	>
		<slot />
	</div>
	<div
		class="flex justify-center"
		style="height: 40px; top: calc(100vh - 50px); width: 100%; position: absolute;"
	>
		<div class="btn-group overflow-x-auto variant-filled">
			<a href="/">
				<MdHome />
			</a>
			<a href="/codes-list">
				<MdFormatListNumbered />
			</a>
			<a href="/settings">
				<MdSettings />
			</a>
		</div>
	</div>
</div>
