<script lang="ts">
    let {nombre, valor, puntaje = $bindable(), dynamic} = $props<string, number, number, boolean | undefined>();
</script>

<div class="evaluacion">
    <!-- Mover -->

    <!-- Borrar -->
    <button class="delete-icon" on:click={}>X</button>
    
    <!-- Nombre -->
    <span class="nombre">{nombre}</span>
    
    <!-- Valor -->
    <span class="valor"><span>{valor < 10 ? "0" + valor : valor}%</span></span>
    
    <!-- Puntaje -->
    <div id="puntaje-contenedor">
        {#if dynamic === true}
            <div class="puntaje">
                <input class="puntaje-input" type="number" min=0 max=20 bind:value={puntaje}>
                <span> pts</span>
            </div>
            <!-- <input type="range" min="0" max="20" bind:value={puntaje}> -->
        {:else}
            <span class="puntaje">{puntaje < 10 ? "0" + puntaje : puntaje} pts</span>
        {/if}
    </div>
</div>

<style src="./style.css">
</style>
