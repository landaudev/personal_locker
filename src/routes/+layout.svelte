<script>
	import "fluent-svelte/theme.css";
</script>




<div class="app">
	<main>
		<slot />
	</main>
</div>

<div class="bg"></div>


<style>
:global(html) {
}

:global(body) {
		background: #292b3a;
		height: 100vh;
		width: 100%;
		padding: 0;
		margin: 0;
}

main {
    color: #eee;
		flex: 1;
		display: flex;
		flex-direction: column;
		width: 100%;
		margin: 0 auto;
		box-sizing: border-box;
	}
</style>