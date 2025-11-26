<script lang="ts">
    let {nombre, valor, puntaje, dynamic} = $props();
</script>

<div class="evaluacion">
    <!-- Icono -->
    <span class="delete-icon">X</span>
    
    <!-- Nombre -->
    <span class="nombre">{nombre}</span>
    
    <!-- Valor -->
    <span class="valor"><span style="border: solid 2px white;">{valor || "00"}%</span></span>
    
    <!-- Puntaje -->
    <div id="puntaje-contenedor">
        <span class="puntaje">{puntaje} pts</span>
        {#if dynamic === true}
            <input type="range" min="0" max="20" bind:value={puntaje}>
        {/if}
    </div>
</div>

<style src="./style.css">
</style>
