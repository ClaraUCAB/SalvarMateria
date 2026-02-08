<script lang="ts">
    import { base } from '$app/paths';

	const faviconPath = `${base}/favicon.svg`;
	const globalCssPath = `${base}/global.css`;

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={faviconPath} />
    <link rel="stylesheet" type="text/css" href={globalCssPath}>
</svelte:head>

{@render children()}

