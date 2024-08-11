<script lang="ts">
  import { onMount } from "svelte";
  import * as d3 from "d3";

  let vis; // binding with div for visualization

  var data = [
    { name: "Aries", symbol: "♈", element: "Fire", angle: 30 },
    { name: "Taurus", symbol: "♉", element: "Earth", angle: 60 },
    { name: "Gemini", symbol: "♊", element: "Air", angle: 90 },
    { name: "Cancer", symbol: "♋", element: "Water", angle: 120 },
    { name: "Leo", symbol: "♌", element: "Fire", angle: 150 },
    { name: "Virgo", symbol: "♍", element: "Earth", angle: 180 },
    { name: "Libra", symbol: "♎", element: "Air", angle: 210 },
    { name: "Scorpio", symbol: "♏", element: "Water", angle: 240 },
    { name: "Sagittarius", symbol: "♐", element: "Fire", angle: 270 },
    { name: "Capricorn", symbol: "♑", element: "Earth", angle: 300 },
    { name: "Aquarius", symbol: "♒", element: "Air", angle: 330 },
    { name: "Pisces", symbol: "♓", element: "Water", angle: 360 },
  ];

  // define the radius and angle scales
  let width: number;
  let height: number;
  const margin = {
    top: 20,
    right: 20,
    bottom: 30,
    left: 30,
  };

  onMount(() => {
    redraw();
    window.addEventListener("resize", redraw);
  });

  function redraw(): void {
    // empty vis div
    d3.select(vis).html(null);

    // determine width & height of parent element and subtract the margin
    width =
      d3.select(vis).node().getBoundingClientRect().width -
      margin.left -
      margin.right;
    height =
      d3.select(vis).node().getBoundingClientRect().height !== 0 ? d3.select(vis).node().getBoundingClientRect().height -
      margin.top - margin.bottom : (window.innerHeight * 0.9) - margin.top - margin.bottom;
    
    const size = Math.min(width, height);
    // Use the calculated size for both width and height
    width = size;
    height = size;

    const chartData = {
      circles: [0.5, 0.6, 0.7],
      points: [],
    };
    
    // create svg and create a group inside that is moved by means of margin
    const svg = d3
      .select(vis)
      .append("svg")
      .attr("width", width + margin.left + margin.right)
      .attr("height", height + margin.top + margin.bottom)
      .append("g")
      .attr(
        "transform",
        `translate(${[width / 2 + margin.left, height / 2 + margin.top]})`
      );

    // draw radial axes
    /*svg
      .append("g")
      .selectAll("line")
      .data(angleScale.ticks(30))
      .enter()
      .append("line")
      .attr("x1", 0)
      .attr("y1", 0)
      .attr("x2", (d) => Math.cos(angleScale(d)) * radiusScale(30))
      .attr("y2", (d) => Math.sin(angleScale(d)) * radiusScale(30))
      .attr("stroke", "darkgray");*/

    // define a color scale based on the element
    let colorScale = d3
      .scaleOrdinal()
      .domain(["Fire", "Earth", "Air", "Water"])
      .range(["#ff0000", "#00ff00", "#0000ff", "#00ffff"]);

    // draw circular axes
    svg
      .append("g")
      .selectAll("circle")
      .data([0.4, 0.85, 0.90, 1])
      .enter()
      .append("circle")
      .attr("cx", 0)
      .attr("cy", 0)
      .attr("r", (d) => d * (width/2))
      .style("stroke", "darkgray")
      .style("fill", "none");

    // Append points
    chartData.points = data.map((d) => {
      const angleDegrees = d.angle;
      const angleRadians = (angleDegrees * Math.PI) / 180;
      const x = 0.95 * Math.cos(angleRadians);
      const y = 0.95 * Math.sin(angleRadians);
      return { x, y, name: d.name, symbol: d.symbol };
    });
    svg
      .selectAll(".point")
      .data(chartData.points)
      .enter()
      .append("circle")
      .attr("class", "point")
      .attr("cx", (d) => d.x * width / 2)
      .attr("cy", (d) => d.y * height / 2)
      .attr("r", 5)
      .style("fill", "red");
    
    svg
      .selectAll(".label")
      .data(chartData.points)
      .enter()
      .append("text")
      .attr("class", "label")
      .attr("x", (d) => d.x * width / 2)
      .attr("y", (d) => d.y * height / 2)
      .attr("dy", "0.35em")
      .style("text-anchor", "middle")
      .text((d) => d.symbol);

    // Append radial ticks between the 2nd and 3rd circle
    const majorTickLength = 5;
    const minorTickLength = 4;
    const tickData = Array.from({ length: 36 }, (_, index) => {
      const angleDegrees = 90 + (index * 10); // 10 degrees for major ticks
      const isMajorTick = index % 2 === 0; // Every 5 degrees for minor ticks
      const tickLength = isMajorTick ? majorTickLength : minorTickLength;
      const angleRadians = (angleDegrees * Math.PI) / 180;
      const x1 = 0.85 * Math.cos(angleRadians);
      const y1 = 0.85 * Math.sin(angleRadians);
      const x2 = (0.88 + tickLength) * Math.cos(angleRadians);
      const y2 = (0.88 + tickLength) * Math.sin(angleRadians);
      return { x1, y1, x2, y2 };
    });

    svg
      .selectAll(".tick")
      .data(tickData)
      .enter()
      .append("line")
      .attr("class", "tick")
      .attr("x1", (d) => d.x1 * width / 2)
      .attr("y1", (d) => d.y1 * height / 2)
      .attr("x2", (d) => d.x2 * width / 2)
      .attr("y2", (d) => d.y2 * height / 2)
      .style("stroke", "black");      
  }
</script>

<div id="vis" bind:this={vis}></div>

<style>
  :root {
    --color: #839496;
    --bg-color: #002b36;
    --main-color: #a81c6e;
  }

  
</style>
