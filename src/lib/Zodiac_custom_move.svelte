<script>
  import { onMount, onDestroy } from 'svelte';
  import * as d3 from 'd3';

  let angle = 0; // Initial angle for the radial chart

  onMount(() => {
    const svg = d3.select('#radial-chart');
    
    // Define your radial chart here
    // For example:
    const radialChart = svg.append('rect')
      .attr('x', 75)
      .attr('y', 75)
      .attr('width', 50)
      .attr('height', 50)
      .attr('fill', 'steelblue')
      .call(d3.drag().on('drag', handleDrag));

    // Add event listeners for keyboard input
    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('keyup', handleKeyUp);

    // Function to handle keyboard input
    function handleKeyDown(event) {
      switch (event.key) {
        case 'ArrowLeft':
          rotateChart(-1); // Adjust the angle change as needed
          break;
        case 'ArrowRight':
          rotateChart(1); // Adjust the angle change as needed
          break;
        default:
          // Do nothing for other keys
      }
    }

    // Function to handle keyup event (stop rotation)
    function handleKeyUp(event) {
      if (event.key === 'ArrowLeft' || event.key === 'ArrowRight') {
        rotateChart(0);
      }
    }

    // Function to handle mouse drag
    function handleDrag(event) {
      const newAngle = Math.atan2(event.y - 100, event.x - 100) * (180 / Math.PI);
      rotateChart(newAngle - 90); // Adjust the offset as needed
    }

    // Function to rotate the chart
    function rotateChart(degrees) {
      angle += degrees;
      radialChart.attr('transform', `rotate(${angle}, 100, 100)`);
    }

    // Cleanup event listeners on component destruction
    onDestroy(() => {
      document.removeEventListener('keydown', handleKeyDown);
      document.removeEventListener('keyup', handleKeyUp);
    });
  });
</script>

<style>
  /* Add your CSS styles here */
</style>

<main>
  <svg id="radial-chart" width="200" height="200"></svg>
</main>
