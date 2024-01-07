<script lang="ts">
  import { onMount } from "svelte";
  import { Chart } from "@astrodraw/astrochart";

  let chart;
  let radius;
  onMount(() => {
    var data = {
      planets: {
        Pluto: [63],
        Neptune: [110],
        Uranus: [318],
        Saturn: [201],
        Jupiter: [192],
        Mars: [210],
        Moon: [268],
        Sun: [281],
        Mercury: [312],
        Venus: [330],
      },
      cusps: [296, 350, 30, 56, 75, 94, 116, 170, 210, 236, 255, 274],
    };
    console.log(window.innerWidth + " / " + window.innerHeight);
    radius = Math.min(window.innerWidth, window.innerHeight) * 0.9;
    chart = new Chart("zodiac-wheel", radius, radius);
    console.log("sending radius " + radius);
    chart.radix(data);
    console.log(chart);
  });
</script>

<div id="zodiac-wheel"></div>

<!-- <svelte:component this= id="zodiac-wheel"></svelte:component> -->

<style>
</style>
