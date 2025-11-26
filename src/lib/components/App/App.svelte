<script lang="ts">
    import { onMount } from 'svelte';
    import { flip } from 'svelte/animate';
    import { slide } from 'svelte/transition';

    import Evaluacion from '$lib/components/Evaluacion/Evaluacion.svelte';

    let controlsForm;
    let nombreEvaluacionInput;
    let valorEvaluacionInput;
    let puntajeInput;

    interface IEvaluacion {
        nombre: string;
        valor: number;
        puntaje: number;
        dynamic?: boolean;
    }
    
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

    let evaluaciones: IEvaluacion[] = $state<IEvaluacion[]>([]);

    let notaReal: number = $state(0);
    let notaRedondeada: number = $state(0);
    let porcentajeEvaluado: number = $derived(
        evaluaciones.reduce((total, ev) => total + ev.valor, 0)
    );

    function addEvaluacion() {
        const nombre: string = nombreEvaluacionInput.value;
        const valor: number = parseInt(valorEvaluacionInput.value);
        const puntaje: number = parseInt(puntajeInput.value) || 10;
        const dynamic: boolean = puntajeInput.value.length === 0;

        // Reset form
        controlsForm.reset()

        evaluaciones.push({nombre, valor, puntaje, dynamic});
    }

    function swap() {
        let temp = evaluaciones[0];
        evaluaciones[0] = evaluaciones[evaluaciones.length-1];
        evaluaciones[evaluaciones.length-1] = temp;
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
        {#each evaluaciones as evaluacion, index (evaluacion.nombre)}
            <div data-index={index} animate:flip={{duration: 200}} transition:slide>
                <Evaluacion {...evaluacion} />
            </div>
        {/each}
        <button onclick={swap}>Swap</button>
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

