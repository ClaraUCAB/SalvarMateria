<script lang="ts">
    let {nombre, valor, puntaje = $bindable(), dynamic, removeEvaluacion = $bindable(), moveUp = $bindable(), moveDown = $bindable()} = $props<string, number, number, boolean | undefined>();


</script>

<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">
<div class="evaluacion">
    <div id="contenedor-controles">
        <!-- Mover -->
        <div id="contenedor-posicion">
            <div class="up button" onclick={moveUp}></div>
            <div class="down button" onclick={moveDown}></div>
        </div>

        <!-- Borrar -->
        <button class="delete-icon" onclick={() => removeEvaluacion(nombre)}>
            <i class="fa fa-trash-o"></i>
        </button>
    </div>

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
