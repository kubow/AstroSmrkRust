<script lang="ts">
  import { onMount } from "svelte";
  import * as d3 from "d3";

  let vis: HTMLDivElement; // binding with div for visualization

  interface DataPoint {  // typing of below data points
    name: string;
    symbol: string;
    element: string;
    angle: number;
  }

  const data: DataPoint[] = [
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
  let radiusScale = d3.scaleLinear().domain([0, 30]);
  let angleScale = d3
    .scaleLinear()
    .domain([0, 30])
    .range([0, 2 * Math.PI]);

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
    if (d3.select(vis).node().getBoundingClientRect().height == 0) {
      height = width / 2
    } else {
      height = d3.select(vis).node().getBoundingClientRect().height -
        margin.top -
        margin.bottom;
    }
    console.log("width: " + width + "\nheight: "+ height);

    // init scales according to new width & height
    radiusScale.range([0, Math.min(width, height) / 2]);
    angleScale.range([0, 2 * Math.PI]);

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
    svg
      .append("g")
      .selectAll("line")
      .data(angleScale.ticks(30))
      .enter()
      .append("line")
      .attr("x1", 0)
      .attr("y1", 0)
      .attr("x2", (d: number) => Math.cos(angleScale(d)) * radiusScale(30))
      .attr("y2", (d: number) => Math.sin(angleScale(d)) * radiusScale(30))
      .attr("stroke", "darkgray");

    // define a color scale based on the element
    let colorScale = d3
      .scaleOrdinal()
      .domain(["Fire", "Earth", "Air", "Water"])
      .range(["#ff0000", "#00ff00", "#0000ff", "#00ffff"]);

    // draw circular axes
    svg
      .append("g")
      .selectAll("circle")
      .data(radiusScale.ticks(30))
      .enter()
      .append("circle")
      .attr("cx", 0)
      .attr("cy", 0)
      .attr("r", (d: number) => radiusScale(d))
      .attr("fill", "none")
      .attr("stroke", "darkgray");

    // draw data points on the outside border
    svg
      .append("g")
      .selectAll("circle")
      .data(data)
      .enter()
      .append("circle")
      .attr("x", function (d: DataPoint) {
        console.log("taking angle:" + d.angle + "˚");
        console.log(2 * Math.PI * (360 / d.angle) * radiusScale(30));
        return Math.cos(angleScale(d.angle)) * radiusScale(30);
      })
      .attr("y", function (d: DataPoint) {
        return Math.sin(angleScale(d.angle)) * radiusScale(30);
      })
      .attr("r", 10) 
      .style("fill", function (d: DataPoint) { //TODO: find how to do this
        return colorScale(d.element);
      })
      .style("stroke", "black");

    // add labels for the data points
    svg
      .append("g")
      .selectAll("text")
      .data(data)
      .enter()
      .append("text")
      .attr("x", function (d: DataPoint) {
        return Math.cos(angleScale(d.angle)) * (radiusScale(30) + 15);
      })
      .attr("y", function (d: DataPoint) {
        return Math.sin(angleScale(d.angle)) * (radiusScale(30) + 15);
      })
      .attr("text-anchor", function (d: DataPoint) {
        return d.angle > 180 ? "end" : "start";
      })
      .attr("alignment-baseline", "middle")
      .text(function (d: DataPoint) {
        return d.name + " " + d.symbol;
      });
  }
</script>

<div id="vis" bind:this={vis}></div>

<style>
  /*:root {
    --color: #839496;
    --bg-color: #002b36;
    --main-color: #a81c6e;
  }

   svg {
    background-color: var(--bg-color);
  }

  path {
    fill: var(--color);
  }

  circle {
    fill: var(--main-color);
  } */
</style>
