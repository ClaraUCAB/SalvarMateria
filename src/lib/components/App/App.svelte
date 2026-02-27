<script lang="ts">
    import { onMount } from 'svelte';
    import { flip } from 'svelte/animate';
    import { slide } from 'svelte/transition';

    import Evaluacion from '$lib/components/Evaluacion/Evaluacion.svelte';

    let controlsForm: HTMLFormElement;
    let nombreEvaluacionInput: HTMLInputElement;
    let valorEvaluacionInput: HTMLInputElement;
    let puntajeInput: HTMLInputElement;

    interface IEvaluacion {
        nombre: string;
        valor: number;
        puntaje: number;
        dynamic?: boolean;
    }
    

    function porcentajeDeNota(valor: number, puntaje: number, base: number = 20): number {
        return ((puntaje / 20 * 100) * (valor / 100) * (base / 100));
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

    let notaReal: number = $derived((evaluaciones.reduce(
        (total, ev) => total + porcentajeDeNota(ev.valor, ev.puntaje),
    0)) || 0);
    let notaRedondeada: number = $derived(Math.round(notaReal));
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

    function removeEvaluacion(nombre: string) {
        evaluaciones = evaluaciones.filter(element => element.nombre != nombre);
    }

    function swapEvaluacion(i: number, j: number) {
        // Ensure we're not out of bounds
        if (i >= evaluaciones.length || j >= evaluaciones.length) return;
        if (i < 0 || j < 0) return 0;

        const temp = evaluaciones[i];
        evaluaciones[i] = evaluaciones[j];
        evaluaciones[j] = temp;
    }
</script>


<main id="contenedor-app">
    <header id="resumen-notas">
        <div id="nota-acumulada">
            Nota real: {notaReal.toFixed(2)} / 20
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
                <Evaluacion {...evaluacion} bind:puntaje={evaluacion.puntaje} removeEvaluacion={removeEvaluacion} moveUp={() => swapEvaluacion(index, index-1)} moveDown={() => swapEvaluacion(index, index+1)} />
            </div>
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
