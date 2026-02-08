<script lang="ts">
    import { base } from '$app/paths';
	import favicon from '$lib/assets/favicon.svg';
	import '$lib/components/App/style.css';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children()}

