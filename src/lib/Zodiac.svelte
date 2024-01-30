<script lang="ts">
  import { onMount } from "svelte";
  import { Chart } from "@astrodraw/astrochart";

  let chart;
  let radius;
  let planets = {
        Pluto: [0],
        Neptune: [0],
        Uranus: [0],
        Saturn: [0],
        Jupiter: [0],
        Mars: [0],
        Moon: [0],
        Sun: [0],
        Mercury: [0],
        Venus: [0],
      };
  onMount(() => {
    var data = {
      planets: planets,
      cusps: [296, 350, 30, 56, 75, 94, 116, 170, 210, 236, 255, 274],
    };
    console.log(window.innerWidth + " / " + window.innerHeight);
    radius = Math.min(window.innerWidth, window.innerHeight) * 0.9;
    chart = new Chart("zodiac-wheel", radius, radius);
    //console.log("sending radius " + radius);
    chart.radix(data);
    //chart.transit(data);
    //chart.addPointsOfInterest( {"As":[data.cusps[0]],"Ic":[data.cusps[3]],"Ds":[data.cusps[6]],"Mc":[data.cusps[9]]});	
    //chart.aspects();
    console.log(chart);
  });
</script>

<div id="zodiac-wheel"></div>

<!-- <svelte:component this= id="zodiac-wheel"></svelte:component> -->

<style>
  #zodiac-wheel {
    color: var(--main-color);
    background-color: var(--bg-color);
  }
</style>
