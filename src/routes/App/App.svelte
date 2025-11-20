<script lang="ts">
    import { onMount } from 'svelte';

    let controlsForm;
    let nombreEvaluacionInput;
    let valorEvaluacionInput;
    let puntajeInput;
    
    onMount(() => {
        if (valorEvaluacionInput && puntajeInput) {
            valorEvaluacionInput.addEventListener('keypress', (event) => {
                if (event.key === 'Enter' && valorEvaluacionInput.checkValidity()) {
                    event.preventDefault();
                    puntajeInput.focus();
                }
            });
        }
    });

    class Evaluacion {
        nombre: string;
        valor: number;
        puntaje: number | undefined;
        
        constructor(nombre: string, valor: number, puntaje?: number | undefined) {
            this.nombre = nombre;
            this.valor = valor;
            this.puntaje = puntaje;
        }
    }

    let evaluaciones: Evaluacion[] = $state([]);

    let notaReal: number = $state(0);
    let notaRedondeada: number = $state(0);
    let porcentajeEvaluado: number = $state(0);

    function addEvaluacion() {
        let nombre: string = nombreEvaluacionInput.value;
        let valor: number = valorEvaluacionInput.value;
        let puntaje: number | undefined = puntajeInput.value ? puntajeInput.value : undefined;

        // Reset form
        /*nombreEvaluacionInput.value = '';
        valorEvaluacionInput.value = '';
        puntajeInput.value = '';*/
        controlsForm.reset();
        nombreEvaluacionInput.focus();

        evaluaciones.push(new Evaluacion(nombre, valor, puntaje));
    }
</script>


<main id="contenedor-app">
    <header id="resumen-notas">
        <div id="nota-acumulada">
            Nota real: {notaReal} / 20
            <br>
            Nota redondeada: {notaRedondeada} / 20
        </div>
        <div id="porcentaje-evaluado">
            Porcentaje evaluado: {porcentajeEvaluado}%
        </div>
    </header>

    <section id="contenedor-evaluaciones">
        {#each evaluaciones as evaluacion}
            <p><strong>{evaluacion.nombre}:</strong> {evaluacion.puntaje} / {evaluacion.valor}</p>
        {/each}
    </section>

    <footer>
        <form id="controles" onsubmit={addEvaluacion} bind:this={controlsForm}>
            <input
                id="nombre-evaluacion"
                bind:this={nombreEvaluacionInput}
                class="control"
                type="text"
                placeholder="Nombre de la evaluación"
                autofocus
                required >
            <input
                id="valor-evaluacion"
                bind:this={valorEvaluacionInput}
                class="control"
                type="number"
                min="0"
                max="100"
                placeholder="Valor de la evaluación (En porcentaje)"
                required >
            <input
                id="puntaje-obtenido"
                bind:this={puntajeInput}
                class="control"
                type="number"
                min="0"
                max="20"
                placeholder="Puntaje obtenido 0-20 (Opcional)" >
            <input
                id="boton-agregar"
                class="control-submit"
                type="submit"
                value="Agregar" >
        </form>
    </footer>
</main>


<style src="./style.css">
</style>

