<script lang="ts">
  import { Chart } from '@astrodraw/astrochart';

  var data = {
  "planets":{
    "Pluto":[63], 
    "Neptune":[110], 
    "Uranus":[318], 
    "Saturn":[201], 
    "Jupiter":[192], 
    "Mars":[210], 
    "Moon":[268], 
    "Sun":[281],
    "Mercury":[312], 
    "Venus":[330]},
  "cusps":[296, 350, 30, 56, 75, 94, 116, 170, 210, 236, 255, 274]          
  };
  var chart = new Chart( 'zodiac-wheel', 800, 800);

  var radix = chart.radix( data );
</script>
  
<style>

</style>

<div id="zodiac-wheel"></div>

