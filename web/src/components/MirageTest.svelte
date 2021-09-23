<script lang="ts">

console.log("sup");
let movies = [];

fetch("/api/movies").then(res => res.json()).then(res => {
    movies = res;
    console.log(movies);
});

</script>

<ul>
	{#each movies as { id, name, year }, i}
		<li><a target="_blank" href="https://www.youtube.com/watch?v={id}">
			{i + 1}: {name} - {year}
		</a></li>
	{/each}
</ul>
