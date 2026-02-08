<script lang="ts">
	//import favicon from '$lib/assets/favicon.svg';
	//import '$lib/components/App/style.css';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href="favicon.svg" />
    <link rel="stylesheet" type="text/css" href="global.css">
</svelte:head>

{@render children()}

